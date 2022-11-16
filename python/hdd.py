import sys

from cmd.remove import remove
from cmd.start import start
from cmd.stop import stop
from util.helper import print_help, print_logo, print_version
from cmd.init import init
from cmd.list import list_

if __name__ == '__main__':
    argv: list[str] = sys.argv
    if len(argv) <= 1:
        print_logo()
        print_help()
        sys.exit(0)
    # 第一个参数为脚本名称，不作考虑
    del argv[0]
    action = argv[0]
    del argv[0]
    if action == 'init':
        init(argv)
    elif action in ['list', 'ls']:
        list_()
    elif action == 'start':
        start(argv)
    elif action == 'stop':
        stop(argv)
    elif action == 'remove':
        remove(argv)
    elif action == 'help':
        print_help()
    elif action == 'version':
        print_version()
    else:
        print(f"未知参数{action}")
        print_help()
