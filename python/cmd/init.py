import os.path
import shutil
import sys
import yaml
from util.log import logger
from util.config import base_dir, image


def init(argv: list):
    stack = argv[0]
    del argv[0]
    # 检查stack是否存在
    init_stack_exist(stack)
    # 校验参数
    parameter = init_check_parameter(argv=argv)
    # 生成docker-compose文件
    init_build_compose(stack=stack, parameter=parameter)
    init_end(stack=stack)


def init_build_compose(stack: str, parameter: dict[str]):
    _stack_path = base_dir + os.path.sep + stack
    docker_compose = {'version': "3.7"}
    services = {}
    if '-nn' in parameter.keys():
        _key = "namenode"
        services[_key] = init_build_server(
            env_file=init_build_env_file(_stack_path),
            image=image,
            hostname=_key,
            container_name=_key,
            volumes=init_build_volumes(_stack_path),
            ports=["9870:9870"],
            command=["sh", "/run-server.sh", "nn"]
        )

    if '-dn' in parameter.keys():
        num = int(parameter['-dn'])
        for i in range(num):
            _key = "datanode_" + str(i)
            services[_key] = init_build_server(
                env_file=init_build_env_file(_stack_path),
                image=image,
                hostname=_key,
                container_name=_key,
                volumes=init_build_volumes(_stack_path),
                ports=None,
                command=["sh", "/run-server.sh", "dn"]
            )
            del services[_key]["ports"]
    if '-2nn' in parameter.keys():
        _key = "secondarynamenode"
        services[_key] = init_build_server(
            env_file=init_build_env_file(_stack_path),
            image=image,
            hostname=_key,
            container_name=_key,
            volumes=init_build_volumes(_stack_path),
            ports=None,
            command=["sh", "/run-server.sh", "2nn"]
        )
        del services[_key]["ports"]
    if '-rm' in parameter.keys():
        _key = "resourcemanager"
        services[_key] = init_build_server(
            env_file=init_build_env_file(_stack_path),
            image=image,
            hostname=_key,
            container_name=_key,
            volumes=init_build_volumes(_stack_path),
            ports=["8088:8088"],
            command=["sh", "/run-server.sh", "rm"]
        )
    if '-nm' in parameter.keys():
        num = int(parameter['-nm'])
        for i in range(num):
            _key = "nodemanager_" + str(i)
            services[_key] = init_build_server(
                env_file=init_build_env_file(_stack_path),
                image=image,
                hostname=_key,
                container_name=_key,
                volumes=init_build_volumes(_stack_path),
                ports=None,
                command=["sh", "/run-server.sh", "nm"]
            )
            del services[_key]["ports"]
    if '-jh' in parameter.keys():
        _key = "historyserver"
        services[_key] = init_build_server(
            env_file=init_build_env_file(_stack_path),
            image=image,
            hostname=_key,
            container_name=_key,
            volumes=init_build_volumes(_stack_path),
            ports=None,
            command=["sh", "/run-server.sh", "jh"]
        )
        del services[_key]["ports"]
    docker_compose['services'] = services
    print()
    # 将字典写入工作空间
    with open(f"{_stack_path}{os.path.sep}docker-compose.yml", "w") as file:
        file.write(yaml.safe_dump(docker_compose, allow_unicode=True))


def init_end(stack: str):
    print(f"stack: {stack} 初始化完成，使用\n"
          f"\thdd start {stack}\n"
          f"启动stack")


def init_build_env_file(_stack_path) -> list:
    return [f"{_stack_path}{os.path.sep}env/hdd-hdfs.env",
            f"{_stack_path}{os.path.sep}env/hdd-yarn.env"]


def init_build_volumes(_stack_path) -> list:
    return [f"{_stack_path}{os.path.sep}runtime{os.path.sep}core-site.xml:/opt/hadoop/etc/hadoop/core-site.xml",
            f"{_stack_path}{os.path.sep}runtime{os.path.sep}hdfs-site.xml:/opt/hadoop/etc/hadoop/hdfs-site.xml",
            f"{_stack_path}{os.path.sep}runtime{os.path.sep}mapred-site.xml:/opt/hadoop/etc/hadoop/mapred-site.xml",
            f"{_stack_path}{os.path.sep}runtime{os.path.sep}yarn-site.xml:/opt/hadoop/etc/hadoop/yarn-site.xml",
            f"{_stack_path}{os.path.sep}runtime{os.path.sep}capacity-scheduler.xml:/opt/hadoop/etc/hadoop/capacity-scheduler.xml"]


def init_build_server(env_file, image, hostname, container_name, volumes, ports, command) -> dict:
    return {"env_file": env_file, "image": image, "hostname": hostname, "container_name": container_name,
            "volumes": volumes, "ports": ports, "command": command}


def init_check_parameter(argv: list) -> dict[str]:
    # 参数个数为偶数
    if len(argv) % 2 != 0:
        logger.error("参数个数不对齐，请检查输入参数")
        sys.exit(-1)
    param = {}
    for index in range(0, len(argv), 2):
        param[argv[index]] = argv[index + 1]
    # -nn -rm 都不存在报错
    if '-nn' not in param.keys() and '-rm' not in param.keys():
        logger.error("-nn 和 -rm 至少存在一个")
        sys.exit(-1)
    # 校验工作节点dn，nm
    if ('-nn' in param.keys() and '-dn' not in param.keys()) or \
            ('-rm' in param.keys() and '-nm' not in param.keys()) or \
            param['-dn'] == '0' or param['-rm'] == '0':
        logger.warn("当前集群存在无工作节点，建议检查参数")
    # 校验组件高可用，当前版本暂不支持
    if ('-nn' in param.keys() and int(param['-nn']) != 1) or \
            ('-rm' in param.keys() and int(param['-rm']) != 1) or \
            ('-2nn' in param.keys() and int(param['-2nn']) != 1) or \
            ('-jh' in param.keys() and int(param['-jh']) != 1):
        logger.warn("当前版本暂不支持组件高可用")
        sys.exit(-1)
    return param


def init_stack_exist(stack: str):
    """
    校验stack是否存在
    :param stack: stack名称
    """
    # 如果工作空间不存在则创建
    if not os.path.exists(base_dir):
        os.mkdir(base_dir)

    # 再检查stack，不存在创建，存在告警
    _stack_path = base_dir + os.path.sep + stack
    if os.path.exists(_stack_path):
        logger.error(f"stack:[{stack}]已存在")
        sys.exit(-1)
    else:
        os.mkdir(_stack_path)
        # 同时将配置文件拷贝到init中
        shutil.copytree(f"{sys.path[0]}{os.path.sep}init", _stack_path + os.path.sep + "init")
        shutil.copytree(f"{sys.path[0]}{os.path.sep}env", _stack_path + os.path.sep + "env")
