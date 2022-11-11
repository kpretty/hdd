# 启动ssh服务
service ssh start
# 将JAVA_HOME写入hadoop-env.sh中
echo "export JAVA_HOME=$JAVA_HOME" >> "${HADOOP_HOME}"/etc/hadoop/hadoop-env.sh