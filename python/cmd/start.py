import os.path
import sys
import shutil

from cmd.common import start_run_cmd, check_param
from util.config import base_dir
from util.log import logger


def start(argv: list):
    check_param(argv)
    stack = argv[0]
    stack_path = f"{base_dir}{os.path.sep}{stack}"
    runtime = f"{stack_path}{os.path.sep}runtime"
    init = f"{stack_path}{os.path.sep}init"
    docker_compose = f"{stack_path}{os.path.sep}docker-compose.yml"
    # 判断一下stack工作空间是否存在
    if not os.path.exists(stack_path):
        logger.error(f"stack: {stack}不存在")
        sys.exit(-1)
    # 将init配置文件复制到runtime下
    if os.path.exists(runtime):
        shutil.rmtree(runtime)
    shutil.copytree(init, runtime)
    # 执行docker-compose up -d
    print(f"启动stack：{stack}...")
    start_run_cmd(f"docker-compose -p {stack} -f {docker_compose} up -d")
