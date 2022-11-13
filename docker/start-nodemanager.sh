#!/bin/bash
"${HADOOP_HOME}"/sbin/yarn-daemons.sh start nodemanager
echo "NodeManager已启动"