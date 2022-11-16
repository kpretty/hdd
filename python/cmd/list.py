import os.path

from util.config import base_dir


def list_():
    print("HDD Stack:\n")
    listdir = os.listdir(base_dir)
    for i in range(len(listdir)):
        print(f"\t{listdir[i]}")
    print()


if __name__ == '__main__':
    list_()
