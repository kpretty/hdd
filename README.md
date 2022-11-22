# hdd
基于docker一站式hadoop集群管理

# 用法
## python cli
基于python实现的hdd客户端，提供hdd stack全生命周期管理(init、start、stop、upgrade(未完成))。<br/>
<img width="696" alt="image" src="https://user-images.githubusercontent.com/77819741/203190961-ec77143a-107b-4be2-af32-2e9d47fcc189.png">

## rust cli
下一阶段。基于python的hdd客户端存在很多无法忍受的问题，例如：需要python环境、需要依赖一些python的第三方包、效率低。下一阶段将使用rust进行重构
