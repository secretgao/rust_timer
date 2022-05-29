#### 项目介绍
项目 一个定时器的程序分server端和client端这是通过udp 连接的
目前任务只支持curl get异步请求
#### 目录说明
* client 客户端文件
* server 服务端文件
* tools 工具类型  打印时间 记录日志  执行shell  序列化反序列化
* worktable  任务队列
#### 原理介绍
* 启动server服务时 在loop维护一个定时任务队列结构体wokerinfo
```shell
loop {
     sleep(1)  //休息一秒
     接收来自客户端对server端发的消息解析成命令,存入wokerinfo结构体中
     每次循环都把wokerinfo.surplus_time的值进行减一操作,当减到0时会异步执行wokerinfo.command中的命令 同时在server/src 目录中生成记录任务执行日志log.txt
     执行完成之后再把wokerinfo.time的值重新赋值给wokerinfo.surplus_time,进入下次循环
     维护一个删除任务的vec 当有任务要删除的时候加入vec中,每次循环检测vec中有没有要删除的任务
     如果有进行删除操作
}


```
#### 使用教程
git clone 到本地
###### 启动一个终端 
``
cd server/src
cargo run     //启动server
``
######  启动另一个终端
``
cd client/src
cargo run   //启动client
``
在输入  
操作类型|任务id|多久执行s|是否永久执行|任务名称|执行命令
```
add|1|10|true|test_a|curl_www.baidu.com|    //添加一个定时任务id=1，每10s运行一次 永久运行 任务名称test执行的命令是 curl www.baidu.com
add|2|5|true|test_a|curl_www.baidu.com|   //添加一个定时任务id=2  每5s运行一次 永久运行 任务名称test1执行的命令是 curl www.baidu.com
update|2|4|false|test_u|curl_www.baidu.com|  //更新任务id=2 4s之后运行一次 任务名称 test_u 行的命令是 curl www.baidu.com
delete|1|                                    //删除任务id=1的任务
```
#### 待优化问题
* 加入 --help 帮助提示
* server  启动时加载配置信息
   * 启动多个任务队列：hashmap容量定义
   * 删除队列：vec容量定义
   * 线程池：容量
   * ip port 配置 
   * 日志记录路径 及日志名称
  
 * 客户端传入参数校验 整型 字符串 布尔类型等
 * 任务失败重试  
 * 任务超时处理
 * 等ing
