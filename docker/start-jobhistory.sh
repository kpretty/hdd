#!/bin/bash
"${HADOOP_HOME}"/sbin/mr-jobhistory-daemon.sh start historyserver
echo "JobHistory已启动"