# MQTT Broker Command

## 1. 集群状态

MQTT Broker 提供了集群状态查询功能，可以通过命令行工具查看集群的健康状态、节点信息等。

```console
% ./bin/robust-ctl mqtt status
cluster name: example_cluster
node list:
- node1
- node2
- node3
MQTT broker cluster up and running
```

## 2. 用户管理

MQTT Broker 启用了用户验证功能，客户端在发布或订阅消息前，
必须提供有效的用户名和密码以通过验证。
未通过验证的客户端将无法与 Broker 通信。
这一功能可以增强系统的安全性，防止未经授权的访问。

### 2.1 创建用户

创建新的 MQTT Broker 用户。

```console
% ./bin/robust-ctl mqtt user create --username=testp --password=7355608 --is-superuser
Created successfully!
```

### 2.2 用户列表

获取 MQTT Broker 用户列表。

```console
% ./bin/robust-ctl mqtt user list
+----------+--------------+
| username | is_superuser |
+----------+--------------+
| admin    | true         |
+----------+--------------+
| testp    | true         |
+----------+--------------+
```

### 2.3 删除用户

删除指定的 MQTT Broker 用户。

```console
% ./bin/robust-ctl mqtt user delete --username=testp
Deleted successfully!
```

## 3. 发布、订阅消息

### 3.1 发布 MQTT 消息

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 publish --username=admin --password=pwd123 --topic=test/topic1 --qos=0
able to connect: "127.0.0.1:1883"
you can post a message on the terminal:
1
> You typed: 1
2
> You typed: 2
3
> You typed: 3
4
> You typed: 4
5
> You typed: 5
^C>  Ctrl+C detected,  Please press ENTER to end the program.
```

### 3.2 订阅 MQTT 消息

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 subscribe --username=admin --password=pwd123 --topic=test/topic1 --qos=0

able to connect: "127.0.0.1:1883"
subscribe success
payload: 1
payload: 2
payload: 3
payload: 4
payload: 5
^C Ctrl+C detected,  Please press ENTER to end the program.
End of input stream.
```

### 3.3 发布保留消息

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 publish --username=admin --password=pwd123 --topic=\$share/group1/test/topic1 --qos=1 --retained
able to connect: "127.0.0.1:1883"
you can post a message on the terminal:
helloworld!
> You typed: helloworld!
published retained message
```

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 subscribe --username=admin --password=pwd123 --topic=\$share/group1/test/topic1 --qos=0
able to connect: "127.0.0.1:1883"
subscribe success
Retain message: helloworld!
```

## 4. ACL（访问控制列表）管理

### 4.1 创建 ACL

创建新的 ACL 规则。

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 acl create --cluster-name=admin --acl=xxx
able to connect: "127.0.0.1:1883"
Created successfully!
```

### 4.2 删除 ACL

删除已有的 ACL 规则。

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 acl delete --cluster-name=admin --acl=xxx
able to connect: "127.0.0.1:1883"
Deleted successfully!
```

### 4.3 ACL 列表

列出所有已创建的 ACL 规则。

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 acl list
+---------------+---------------+-------+----+--------+------------+
| resource_type | resource_name | topic | ip | action | permission |
+---------------+---------------+-------+----+--------+------------+
```

## 5. 黑名单管理

### 5.1 创建黑名单

创建新的黑名单规则。

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 blacklist create --cluster-name=admin --blacklist=client_id
able to connect: "127.0.0.1:1883"
Created successfully!
```

### 5.2 删除黑名单

删除已有的黑名单规则。

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 blacklist delete --cluster-name=admin --blacklist-type=client_id --resource-name=client1
able to connect: "127.0.0.1:1883"
Deleted successfully!
```

### 5.3 黑名单列表

列出所有已创建的黑名单规则。

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 blacklist list
+----------------+---------------+----------+------+
| blacklist_type | resource_name | end_time | desc |
+----------------+---------------+----------+------+
```

## 6. 开启慢订阅功能

### 6.1 开启/关闭慢订阅

慢订阅统计功能主要是为了在消息到达 Broker 后，
Broker 来计算完成消息处理以及传输整个流程所消耗的时间(时延),
如果时延超过阈值，我们就会记录一条相关的信息在集群慢订阅日志当中，
运维人员可以通过命令查询整个集群下的慢订阅记录信息，
通过慢订阅信息来解决。

- 开启慢订阅

```console
% ./bin/robust-ctl mqtt slow-sub --enable=true
The slow subscription feature has been successfully enabled.
```

- 关闭慢订阅

```console
% ./bin/robust-ctl mqtt slow-sub --enable=false
The slow subscription feature has been successfully closed.
```

### 6.2 查询慢订阅记录

当我们启动了慢订阅统计功能之后, 集群就开启慢订阅统计功能，
这样我们可以通过对应的命令来去查询对应的慢订阅记录，
如果我们想要查看慢订阅记录，客户端可以输入如下命令

```console
% ./bin/robust-ctl mqtt slow-sub --list
+-----------+-------+----------+---------+-------------+
| client_id | topic | sub_name | time_ms | create_time |
+-----------+-------+----------+---------+-------------+
```

### 6.3 排序功能

如果想要获取更多的慢订阅记录，
并且想要按照从小到大的顺序进行升序排序，
那么可以使用如下的命令

```console
% ./bin/robust-ctl mqtt slow-sub --list=200 --sort=asc
+-----------+-------+----------+---------+-------------+
| client_id | topic | sub_name | time_ms | create_time |
+-----------+-------+----------+---------+-------------+
```

### 6.4 过滤查询功能

对于慢订阅查询，我们同样支持筛选查询功能，我们支持使用 topic,
sub_name 以及 client_id 的方式来获取不同字段过滤后的结果，
其结果默认从大到小倒序排序，参考使用命令如下

```console
% ./bin/robust-ctl mqtt slow-sub --topic=topic_test1 --list=200
+-----------+-------+----------+---------+-------------+
| client_id | topic | sub_name | time_ms | create_time |
+-----------+-------+----------+---------+-------------+
```

## 7. 主题重写规则

很多物联网设备不支持重新配置或升级，修改设备业务主题会非常困难。

主题重写功能可以帮助使这种业务升级变得更容易：通过设置一套规则，它可以在订阅、发布时改变将原有主题重写为新的目标主题。

### 7.1 创建主题重写规则

```console
% ./bin/robust-ctl mqtt topic-rewrite-rule create --action=xxx --source-topic=xxx --dest-topic=xxx --regex=xxx
Created successfully!
```

### 7.2 删除主题重写规则

```console
% ./bin/robust-ctl mqtt topic-rewrite-rule delete --action=xxx --source-topic=xxx
Deleted successfully!
```

## 8. 连接抖动检测

在黑名单功能的基础上，支持自动封禁那些被检测到短时间内频繁登录的客户端，并且在一段时间内拒绝这些客户端的登录，以避免此类客户端过多占用服务器资源而影响其他客户端的正常使用。

- 开启连接抖动检测

```console
% ./bin/robust-ctl mqtt flapping-detect --enable=true --window-time=1 --max-client-connections=15 --ban-time=5
The flapping detect feature has been successfully enabled.
```

- 关闭连接抖动检测

```console
% ./bin/robust-ctl mqtt flapping-detect --enable=false
The flapping detect feature has been successfully closed.
```

## 9. 连接列表

连接列表命令用于查询 MQTT Broker 当前的连接状态，提供连接 ID、类型、协议、源地址等相关信息。

```console
% ./bin/robust-ctl mqtt list-connection
connection list:
+---------------+-----------------+----------+-------------+------+
| connection_id | connection_type | protocol | source_addr | info |
+---------------+-----------------+----------+-------------+------+
```

## 10. 主题列表

主题列表命令用于查询 MQTT Broker 当前的主题状态，提供主题 ID、主题名称、集群名称以及是否包含保留消息等相关信息。

```console
% ./bin/robust-ctl mqtt list-topic
topic list result:
+----------------------------------+------------------------------------+--------------+---------------------------+
| topic_id                         | topic_name                         | cluster_name | is_contain_retain_message |
+----------------------------------+------------------------------------+--------------+---------------------------+
| d586681e0b334dc4909b4189c09d6383 | $SYS/brokers                       | mqtt-broker  | false                     |
+----------------------------------+------------------------------------+--------------+---------------------------+
| 0da56ebfdec04d21b068373aad57b29c | $SYS/brokers/172.20.10.13/uptime   | mqtt-broker  | false                     |
+----------------------------------+------------------------------------+--------------+---------------------------+
| 21f81dd57a68436cad4f4ca4405be6cb | $SYS/brokers/172.20.10.13/version  | mqtt-broker  | false                     |
+----------------------------------+------------------------------------+--------------+---------------------------+
| 9fb1a15794b8475e9d5e07a4fd29b5ae | $SYS/brokers/172.20.10.13/sysdescr | mqtt-broker  | false                     |
+----------------------------------+------------------------------------+--------------+---------------------------+
| f25b9b1b94944f0b97a118c03b3f72bd | $SYS/brokers/172.20.10.13/datetime | mqtt-broker  | false                     |
+----------------------------------+------------------------------------+--------------+---------------------------+
```

## 11. 连接器管理

连接器允许 MQTT Broker 与外部系统连接，实现不同平台之间的数据交换。

### 11.1 创建连接器

创建新的连接器。

```console
% ./bin/robust-ctl mqtt connector create --connector-name=my-connector --connector-type=kafka --config='{"bootstrap_servers":"localhost:9092","topic":"test-topic","key":"test-key"}' --topic-id=1
Created successfully!
```

### 11.2 列出连接器

列出系统中的所有连接器。

```console
% ./bin/robust-ctl mqtt connector list --connector-name=my-connector
connector list result:
+--------------+----------------+----------------+------------------------------------------------------------------------------+----------+--------+-----------+-------------+-------------+
| cluster name | connector name | connector type | connector config                                                             | topic id | status | broker id | create time | update time |
+--------------+----------------+----------------+------------------------------------------------------------------------------+----------+--------+-----------+-------------+-------------+
| mqtt-broker  | my-connector   | Kafka          | {"bootstrap_servers":"localhost:9092","topic":"test-topic","key":"test-key"} | 1        | Idle   | 0         | 1746434176  | 1746434176  |
+--------------+----------------+----------------+------------------------------------------------------------------------------+----------+--------+-----------+-------------+-------------+
```

### 11.3 更新连接器

更新现有的连接器。

```console
% ./bin/robust-ctl mqtt connector update --connector='{"cluster_name":"example","connector_name":"my-connector","connector_type":"Kafka","config":"{\"bootstrap_servers\":\"localhost:9092\",\"topic\":\"test-topic-update\",\"key\":\"test-key-update\"}","topic_id":"1","status":"Running","broker_id":null,"create_time":1710000000,"update_time":1710000000}'
Updated successfully!
```

### 11.4 删除连接器

删除现有的连接器。

```console
% ./bin/robust-ctl mqtt connector delete --connector-name=my-connector
Deleted successfully!
```

## 12. 模式管理

模式定义了消息的结构和格式，确保数据一致性和验证。

### 12.1 创建模式

创建新的模式。

```console
% ./bin/robust-ctl mqtt schema create --cluster-name=example --schema-name=temperature_schema --schema-type=json --schema="{\"type\":\"object\",\"properties\":{\"temperature\":{\"type\":\"number\"},\"timestamp\":{\"type\":\"integer\"}}}" -desc="create"
Created successfully!
```

### 12.2 列出模式

列出系统中的所有模式。

```console
% ./bin/robust-ctl mqtt schema list
schema list result:
cluster name: example_cluster
schema name: temperature_schema
schema type: json
schema desc: create
schema: {"type":"object","properties":{"temperature":{"type":"number"},"timestamp":{"type":"integer"}}}
```

### 12.3 更新模式

更新现有的模式。

```console
% ./bin/robust-ctl mqtt schema update --cluster-name=example --schema-name=temperature_schema --schema="{\"type\":\"object\",\"properties\":{\"temperature\":{\"type\":\"number\"},\"timestamp\":{\"type\":\"integer\"},\"unit\":{\"type\":\"string\"}}}" --desc="update"
Updated successfully!
```

### 12.4 删除模式

删除现有的模式。

```console
% ./bin/robust-ctl mqtt schema delete --cluster-name=example --schema-name=temperature_schema
Deleted successfully!
```

### 12.5 绑定模式

将模式绑定到主题。

```console
% ./bin/robust-ctl mqtt schema bind --cluster-name=example --schema-name=temperature_schema --topic-id=1
Created successfully!
```

### 12.6 解绑模式

从主题解绑模式。

```console
% ./bin/robust-ctl mqtt schema unbind --cluster-name=example --schema-name=temperature_schema --topic-id=1
Deleted successfully!
```

### 12.7 列出绑定的模式

列出所有模式绑定。

```console
% ./bin/robust-ctl mqtt schema list-bind
bind schema list result:
cluster name: example_cluster
schema name: temperature_schema
schema type: json
schema desc: update
schema: {"type":"object","properties":{"temperature":{"type":"number"},"timestamp":{"type":"integer"}}}
```

## 13. 自动订阅规则

自动订阅规则允许 Broker 根据预定义规则自动将客户端订阅到特定主题。

### 13.1 设置自动订阅规则

创建或更新自动订阅规则。

```console
% ./bin/robust-ctl mqtt auto-subscribe-rule set --topic=test/topic --qos=1 --no-local --retain-as-published --retained-handling=1
Created successfully!
```

### 13.2 列出自动订阅规则

列出所有自动订阅规则。

```console
% ./bin/robust-ctl mqtt auto-subscribe-rule list
+------------+-----+----------+---------------------+-------------------+
| topic      | qos | no_local | retain_as_published | retained_handling |
+------------+-----+----------+---------------------+-------------------+
| test/topic | 1   | true     | true                | 1                 |
+------------+-----+----------+---------------------+-------------------+
```

### 13.3 删除自动订阅规则

删除现有的自动订阅规则。

```console
% ./bin/robust-ctl mqtt auto-subscribe-rule delete --topic=test/topic
Deleted successfully!
```
