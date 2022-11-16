import os

# logo
logo_up = "      ___           ___           ___     \n" + \
          "     /\\__\\         /\\  \\         /\\  \\\n" + \
          "    /:/  /        /::\\  \\       /::\\  \\\n" + \
          "   /:/__/        /:/\\:\\  \\     /:/\\:\\  \\ \n"

logo_mid = "  /::\\  \\ ___   /:/  \\:\\__\\   /:/  \\:\\__\\\n" + \
           " /:/\\:\\  /\\__\\ /:/__/ \\:|__| /:/__/ \\:|__|\n" + \
           " \\/__\\:\\/:/  / \\:\\  \\ /:/  / \\:\\  \\ /:/  /\n" + \
           "      \\::/  /   \\:\\  /:/  /   \\:\\  /:/  /  \n"

logo_down = "      /:/  /     \\:\\/:/  /     \\:\\/:/  /\n" + \
            "     /:/  /       \\::/__/       \\::/__/\n" + \
            "     \\/__/         ~~            ~~\n"
# 版本信息
version = "v0.0.1"

# 镜像信息
image = "hdd/hadoop-base"

hdd_desc = "HDD CLI is a developer tool used to manage local development stacks\n\n" + \
           "This tool automates creation of stacks with many infrastructure components which\n" + \
           "would otherwise be a time consuming manual task. It also wraps docker compose\n" + \
           "commands to manage the lifecycle of stacks.\n\n"

hdd_start = "To get started run: hdd init\n\n" + \
            "Usage:\n  hdd [command]\n\n" + \
            "Available Commands:\n" + \
            "  help        帮助命令\n" + \
            "  info        查看stack详细信息[未完成]\n" + \
            "  init        初始化一个stack\n" + \
            "  list        查看所有stack\n" + \
            "  logs        查看某个stack日志信息[未完成]\n" + \
            "  ls          查看所有stack\n" + \
            "  remove      移除stack\n" + \
            "  start       启动stack\n" + \
            "  stop        停止stack\n" + \
            "  version     打印版本信息\n\n"

# hdd工作空间
base_dir = os.path.expanduser('~') + os.sep + ".hdd"
