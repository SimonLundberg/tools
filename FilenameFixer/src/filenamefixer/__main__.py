import sys
from .filenamefixer import fix_html_encoding

def main():
    for line in sys.stdin:
        fix_html_encoding(line.rstrip())


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        exit(0)