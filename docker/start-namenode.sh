#!/bin/bash
FORMAT_FLAG=/formatted.hdp
# 判断是否已经初始化
if [ ! -f ${FORMAT_FLAG} ]; then
  echo "开始格式化NameNode"
  "${HADOOP_HOME}"/bin/hdfs namenode -format
  touch ${FORMAT_FLAG}
fi
"${HADOOP_HOME}"/sbin/hadoop-daemons.sh start namenode
echo "NameNode已启动"