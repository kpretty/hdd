from util import config
from util.color import *


def print_logo():
    print(f"{RED}{config.logo_up}{BLUE}{config.logo_mid}{YELLOW}{config.logo_down}{REST}")


def print_help():
    print(config.hdd_desc)
    print(config.hdd_start)


def print_version():
    print(f"{config.version}")


if __name__ == '__main__':
    print_logo()
    print_help()
