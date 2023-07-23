import tkinter as tk
from tkinter import filedialog
import os

gui_dir = os.path.dirname(__file__)
gui_dir = os.path.abspath(gui_dir)
root = os.path.dirname(gui_dir)
picosakura = os.path.join(root, "picosakura")

def open_file_dialog():
    # ファイル選択ダイアログを表示し、選択されたファイルのパスを取得する
    file_path = filedialog.askopenfilename()
    if file_path == "": return
    cmd = f'"{picosakura}" "{file_path}" -wav'
    os.system(cmd)

# メインウィンドウを作成
root = tk.Tk()
root.title("ファイル選択ダイアログ")

# ボタンを作成
button = tk.Button(root, text="ファイルを選択", command=open_file_dialog)
button.pack(pady=20)

# メインループを開始
root.mainloop()
