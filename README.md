<div align='center' ><font size='70'>HDD：Hadoop for Docker</font></div>

hdd是基于docker的一站式hadoop管理工具，本意是为了方便初学者快速搭建hadoop分布式集群，可以实现一台电脑多个hadoop集群且互相隔离，hdd提供简单命令实现集群的初始化、启动、停止、删除

## 使用文档

环境准备：docker、docker-compose

```shell
./hdd
      ___           ___           ___
     /\__\         /\  \         /\  \
    /:/  /        /::\  \       /::\  \
   /:/__/        /:/\:\  \     /:/\:\  \
  /::\  \ ___   /:/  \:\__\   /:/  \:\__\
 /:/\:\  /\__\ /:/__/ \:|__| /:/__/ \:|__|
 \/__\:\/:/  / \:\  \ /:/  / \:\  \ /:/  /
      \::/  /   \:\  /:/  /   \:\  /:/  /
      /:/  /     \:\/:/  /     \:\/:/  /
     /:/  /       \::/__/       \::/__/
     \/__/         ~~            ~~
HDD CLI is a developer tool used to manage local development stacks

This tool automates creation of stacks with many infrastructure components which
would otherwise be a time consuming manual task. It also wraps docker compose
commands to manage the lifecycle of stacks.


To get started run: hdd init

Usage:
  hdd [command]

Available Commands:
 help        帮助命令
 info        查看stack详细信息[未完成]
 init        初始化一个stack
 list        查看所有stack
 logs        查看某个stack日志信息[未完成]
 ls          查看所有stack
 remove      移除stack
 start       启动stack
 stop        停止stack
 status      查看stack状态 
 version     打印版本信息
```

查看版本信息

```shell
./hdd version
```

初始化集群

```shell
./hdd init dev -nn 1 -dn 3 -rm 1 -nm 3 -2nn 1 -jh 1
```

注：

- -nn：指定namenode个数
- -dn：指定datanode个数
- -2nn：指定secondarynamenode个数
- -rm：指定resourcemanager个数
- -nm：指定nodemanager个数
- -jh：指定jobhistory个数

> 当前版本暂不支持HA，后续会考虑

启动集群

```shell
./hdd start dev
```

停止集群

```shell
./hdd stop dev
```

删除集群

```shell
./hdd remove dev
```

查看集群状态

```shell
./hdd status dev
```

查看stack

```shell
./hdd list
```

## 通过源码构建

1. 准备rust环境，参考官网：https://www.rust-lang.org/tools/install
2. 下载源码：`git clone https://github.com/kpretty/hdd.git; cd hdd`
3. 编译源码：`cargo build --release;cd target/release`
4. 将脚本`hdd`和项目根目录的`init`、`env`文件拷贝出来即可

## 后续计划

- [ ] 组件高可用
- [ ] 组件动态扩缩容
- [ ] 更细腻度的自定义，如镜像版本、组件端口
- [ ] 支持更多的hadoop版本，至少覆盖hadoop2.x和hadoop3.x
