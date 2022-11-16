import subprocess
import sys

from util.log import logger


def start_run_cmd(cmd: str):
    p = subprocess.Popen(cmd, stdin=subprocess.PIPE, stderr=subprocess.PIPE,
                         stdout=subprocess.PIPE, universal_newlines=True, shell=True, bufsize=1)
    # 实时输出
    while True:
        line = p.stdout.readline()
        print(line, end='')
        if subprocess.Popen.poll(p) == 0:  # 判断子进程是否结束
            break


def check_param(argv: list):
    if len(argv) != 1:
        logger.error("参数错误，可能有价值的参考 hdd start|stop|remove [stack]")
        sys.exit(-1)
