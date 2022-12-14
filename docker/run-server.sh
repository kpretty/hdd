#!/bin/bash
# 初始化系统服务和做一些通用的处理
sh /init-server.sh
# 根据参数启动不同服务：nn,dn,2nn,rm,nm,jh
case $1 in
nn)
  sh /start-namenode.sh
  ;;
dn)
  sh /start-datanode.sh
  ;;
2nn)
  sh /start-secondarynamenode.sh
  ;;
rm)
  sh /start-resourcemanager.sh
  ;;
nm)
  sh /start-nodemanager.sh
  ;;
jh)
  sh /start-jobhistory.sh
  ;;
*)
  echo "ERROR：未知参数"
  exit 1
  ;;
esac
while :
do
    # 死循环，让容器持续运行
    sleep 1
done

# todo 后续计划将对应服务的日志发送到容器的stdout，使得docker logs可以看到日志[再议]