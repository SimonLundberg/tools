from urllib import parse
import os

def fix_html_encoding(filepath: str):
    new_path = parse.unquote(filepath)
    os.rename(filepath, new_path)
