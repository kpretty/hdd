#!/bin/bash
"${HADOOP_HOME}"/sbin/hadoop-daemons.sh start datanode
echo "DataNode已启动"