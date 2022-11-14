#!/bin/bash
"${HADOOP_HOME}"/sbin/yarn-daemons.sh start resourcemanager
echo "ResourceManager已启动"