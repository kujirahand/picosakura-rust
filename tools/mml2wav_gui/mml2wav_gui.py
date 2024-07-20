import tkinter as tk
from tkinter import filedialog


def select_file():
    file_path = filedialog.askopenfilename()
    if file_path:
        show_alert("選択されたファイルパス: {}".format(file_path))
    else:
        show_alert("ファイルが選択されませんでした。")


def show_alert(message):
    alert_window = tk.Toplevel(root)
    alert_window.title("アラート")
    alert_label = tk.Label(alert_window, text=message, padx=10, pady=5)
    alert_label.pack()
    ok_button = tk.Button(alert_window, text="OK",
                          command=alert_window.destroy)
    ok_button.pack()


# メインウィンドウを作成
root = tk.Tk()
root.title("ファイル選択")

# ファイル選択ボタンを作成
select_button = tk.Button(root, text="ファイルを選択", command=select_file)
select_button.pack(pady=20)

# アプリケーションのループを開始
root.mainloop()
