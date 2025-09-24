import os
import threading
import TkEasyGUI as eg

DIR_SCRIPT = os.path.dirname(os.path.abspath(__file__))
PICOSAKURA = os.path.join(DIR_SCRIPT, "bin", "picosakura")
FONT = os.path.join(DIR_SCRIPT, "bin", "fonts", "TimGM6mb.sf2")

filename = None

def main():
    """Simple GUI main loop.

    Creates the window layout, handles button events (New, Load, Save,
    Play, Stop, Export) and updates the editor contents. This function
    runs the event loop until the window is closed.

    The function mutates the module-level `filename` to track the
    currently-loaded or saved file path.
    """
    global filename
    layout = [
        [
            eg.Button("New", key="New"),
            eg.Label("|"),
            eg.Button("Load", key="Load"),
            eg.Button("Save", key="Save"),
            eg.Label("|"),
            eg.Button("♫ Play", key="Play"),
            eg.Button("□ Stop", key="Stop"),
            eg.Label("|"),
            eg.Button("Export", key="Export"),
        ],
        [eg.Multiline("", size=(80, 30), key="-edit")],
    ]

    window = eg.Window("PicoGUI", layout=layout)

    while window.is_running():
        event, values = window.read()
        print(f"Event: {event}, Values: {values}")
        if event in (None,  eg.WIN_CLOSED):
            break
        if event == "New":
            if eg.popup_yes_no("Are you sure you want to create a new file? Unsaved changes will be lost.") == "Yes":
                window["-edit"].update("")
                filename = None
                continue
        if event == "Load":
            filename = eg.popup_get_file("Load File", no_window=True)
            if filename:
                try:
                    with open(filename, "r") as f:
                        content = f.read()
                    window["-edit"].update(content)
                except Exception as e:
                    eg.popup_error(f"Error loading file: {e}")
            continue
        if event == "Save":
            if filename is None:
                filename = eg.popup_get_file("Save File", save_as=True, no_window=True)
            if filename:
                try:
                    with open(filename, "w") as f:
                        f.write(values["-edit"])
                except Exception as e:
                    eg.popup_error(f"Error saving file: {e}")
            continue
        if event == "Play":
            play_audio(window)
            continue
        if event == "Stop":
            stop_audio(window)
            continue
        if event == "Export":
            export_audio(window)

    window.close()

def play_audio(window):
    """Play the MML text currently in the editor.

    Behavior:
    - Reads the MML text from the editor widget.
    - Writes it to `filename`, or a temporary file if no filename is set.
    - Constructs the picosakura command and starts it in a background
      thread so the GUI remains responsive.

    Any file or execution errors are shown to the user via a popup.
    """
    global filename
    mml = window["-edit"].get()
    # save mml
    if filename is None:
        filename = os.path.join(DIR_SCRIPT, "temp.mml")
    try:
        with open(filename, "w", encoding="utf-8") as f:
            f.write(mml)
    except Exception as e:
        eg.popup_error(f"Error playing audio: {e}")
        return
    # play
    cmd = f"\"{PICOSAKURA}\" -s \"{FONT}\" \"{filename}\""
    print(f"Executing command: {cmd}")
    threading.Thread(target=run_command, args=(cmd,)).start()

def run_command(cmd):
    """Run the given shell command synchronously.

    Note: this function is intended to be executed in a separate
    background thread so that long-running commands don't block the
    GUI event loop.
    """
    os.system(cmd)

def stop_audio(window):
    """Stop any currently-running picosakura process.

    Uses platform-appropriate commands:
    - On Windows, calls `taskkill` to terminate `picosakura.exe`.
    - On POSIX systems, uses `pkill picosakura` to stop the process.

    This is a best-effort stop; the commands used depend on the host
    environment having `taskkill` or `pkill` available.
    """
    # kill process
    if os.name == 'nt':
        os.system("taskkill /f /im picosakura.exe")
    else:
        os.system("pkill picosakura")

def export_audio(window):
    """Export the current MML as a WAV file.

    Workflow:
    - Prompt the user for an export file path.
    - Save the editor MML to `filename` (or a temporary file if unset).
    - Run picosakura with the `-w` option to write the WAV output to the
      chosen file. The export command is executed in a background thread
      so the GUI stays responsive. The user is notified that export has
      started and should check the output file when it's finished.
    """
    global filename
    mml = window["-edit"].get()
    eg.popup("Please choose the export .WAV file location.")
    export_filename = eg.popup_get_file("Export File", save_as=True, no_window=True, default_extension=".wav", file_types=(("WAV Files", "*.wav"), ("All Files", "*.*")))
    if not export_filename:
        return
    # save mml
    if filename is None:
        filename = os.path.join(DIR_SCRIPT, "temp.mml")
    try:
        with open(filename, "w", encoding="utf-8") as f:
            f.write(mml)
    except Exception as e:
        eg.popup_error(f"Error exporting audio: {e}")
        return
    # export
    cmd = f"\"{PICOSAKURA}\" -w \"{export_filename}\" -s \"{FONT}\" \"{filename}\""
    print(f"Executing command: {cmd}") 
    threading.Thread(target=run_command, args=(cmd,)).start()
    eg.popup("Export started. Please check the output file when done.")

if __name__ == "__main__":
    main()
