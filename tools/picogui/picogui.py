import os
import threading
import TkEasyGUI as eg

DIR_SCRIPT = os.path.dirname(os.path.abspath(__file__))
PICOSAKURA = os.path.join(DIR_SCRIPT, "bin", "picosakura")
FONT = os.path.join(DIR_SCRIPT, "bin", "fonts", "TimGM6mb.sf2")

filename = None

def main():
    global filename
    layout = [
        [
            eg.Button("New"),
            eg.Button("Load"),
            eg.Button("Save"),
            eg.Label("|"),
            eg.Button("Play"),
            eg.Button("Stop"),
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

    window.close()

def play_audio(window):
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
    os.system(cmd)

def stop_audio(window):
    # kill process
    if os.name == 'nt':
        os.system("taskkill /f /im picosakura.exe")
    else:
        os.system("pkill picosakura")


if __name__ == "__main__":
    main()
