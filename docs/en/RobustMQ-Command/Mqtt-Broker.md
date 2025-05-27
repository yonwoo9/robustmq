# MQTT Broker Command

## 1. Cluster status

MQTT offered cluster status query function, which allows users to check the health status of the cluster, node information, and more through the command line tool.

```console
% ./bin/robust-ctl mqtt status
cluster name: example_cluster
node list:
- node1
- node2
- node3
MQTT broker cluster up and running
```

## 2. User Management

MQTT Broker has enabled user authentication. Clients must provide valid usernames and passwords before publishing or subscribing to messages to pass the authentication. Clients that fail authentication will not be able to communicate with the Broker. This feature enhances system security and prevents unauthorized access.

### 2.1 Create User

Create a new MQTT Broker user.

```console
% ./bin/robust-ctl mqtt user create --username=testp --password=7355608 --is-superuser
Created successfully!
```

### 2.2 List User

Get MQTT Broker user list

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

### 2.3 Delete User

Delete a MQTT Broker user

```console
% ./bin/robust-ctl mqtt user delete --username=testp
Deleted successfully!
```

## 3. Pub & Sub

### 3.1 publish

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

### 3.2 subscribe

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

### 3.3 Pub retain message

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 publish --username=admin --password=pwd123 --topic=\$share/group1/test/topic1 --qos=1 --retained
able to connect: "127.0.0.1:1883"
you can post a message on the terminal:
helloworld!
> You typed: helloworld!
published retained message
```

## 4. ACL (Access Control List) Management

### 4.1 Create ACL

Create a new ACL rule.

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 acl create --cluster-name=admin --acl=xxx
able to connect: "127.0.0.1:1883"
Created successfully!
```

### 4.2 Delete ACL

Delete an existing ACL rule.

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 acl delete --cluster-name=admin --acl=xxx
able to connect: "127.0.0.1:1883"
Deleted successfully!
```

### 4.3 ACL List

List all created ACL rules.

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 acl list
+---------------+---------------+-------+----+--------+------------+
| resource_type | resource_name | topic | ip | action | permission |
+---------------+---------------+-------+----+--------+------------+
```

## 5. Blacklist Management

### 5.1 Create Blacklist

Create a new blacklist rule.

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 blacklist create --cluster-name=admin --blacklist=client_id
able to connect: "127.0.0.1:1883"
Created successfully!
```

### 5.2 Delete Blacklist

Delete an existing blacklist rule.

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 blacklist delete --cluster-name=admin --blacklist-type=client_id --resource-name=client1
able to connect: "127.0.0.1:1883"
Deleted successfully!
```

### 5.3 Blacklist List

List all created blacklist rules.

```console
% ./bin/robust-ctl mqtt --server=127.0.0.1:1883 blacklist list
+----------------+---------------+----------+----------------+
| blacklist_type | resource_name | end_time | blacklist_type |
+----------------+---------------+----------+----------------+
```

## 6. Slow Subscription

The slow subscription statistics function is mainly used to calculate the time (latency) it takes for the Broker to complete message processing and transmission after the message arrives at the Broker. If the latency exceeds the threshold, we will record a related piece of information in the cluster's slow subscription log. Operations personnel can query slow subscription records across the entire cluster using commands to address issues based on this information

### 6.1 Enable Slow Subscription

- Enable slow subscription

```console
% ./bin/robust-ctl mqtt slow-sub --enable=true
The slow subscription feature has been successfully enabled.
```

- Close slow subscription

```console
% ./bin/robust-ctl mqtt slow-sub --enable=false
The slow subscription feature has been successfully closed.
```

### 6.2 View slow subscription records

After enabling the slow subscription statistics function, the cluster begins recording slow subscriptions. To query corresponding slow subscription records, clients can enter the following command:

```console
% ./bin/robust-ctl mqtt slow-sub --list
+-----------+-------+----------+---------+-------------+
| client_id | topic | sub_name | time_ms | create_time |
+-----------+-------+----------+---------+-------------+
```

### 6.3 Sorting Functionality

To obtain more slow subscription records and sort them in ascending order from smallest to largest, you can use the following command:

```console
% ./bin/robust-ctl mqtt slow-sub --list=200 --sort=asc
+-----------+-------+----------+---------+-------------+
| client_id | topic | sub_name | time_ms | create_time |
+-----------+-------+----------+---------+-------------+
```

### 6.4 Filtering Query Functionality

For slow subscription queries, filtering queries are also supported. You can retrieve filtered results by fields such as topic, sub_name, and client_id. By default, the results are sorted in descending order from largest to smallest. Refer to the usage command below:

```console
% ./bin/robust-ctl mqtt slow-sub --topic=topic_test1 --list=200
+-----------+-------+----------+---------+-------------+
| client_id | topic | sub_name | time_ms | create_time |
+-----------+-------+----------+---------+-------------+
```

## 7. Topic Rewrite Rule

Many IoT devices do not support reconfiguration or upgrades, making it very difficult to modify the device's business topics.

The topic rewriting feature can help make such business upgrades easier: by setting up a set of rules, it can change the original topic to a new target topic during subscription and publishing.

### 7.1 Create Topic Rewrite Rule

Create a new topic rewrite rule.

```console
% ./bin/robust-ctl mqtt topic-rewrite-rule create --action=xxx --source-topic=xxx --dest-topic=xxx --regex=xxx
Created successfully!
```

### 7.2 Delete Topic Rewrite Rule

Delete an existing topic rewrite rule.

```console
% ./bin/robust-ctl mqtt topic-rewrite-rule delete --action=xxx --source-topic=xxx
Deleted successfully!
```

## 8. Flapping Detection

Based on the blacklist feature, supports automatically blocking clients that are detected to log in frequently within a short period. These clients are denied login for a certain period to prevent them from excessively occupying server resources and affecting the normal use of other clients.

- Enable flapping detection

```console
% ./bin/robust-ctl mqtt flapping-detect --enable=true --window-time=1 --max-client-connections=15 --ban-time=5
The flapping detect feature has been successfully enabled.
```

- Close flapping detection

```console
% ./bin/robust-ctl mqtt flapping-detect --enable=false
The flapping detect feature has been successfully closed.
```

## 9. Connection List

The connection list command is used to query the current connection status of the MQTT Broker. It provides information about the connection ID, type, protocol, source address, and other relevant details.

```console
% ./bin/robust-ctl mqtt list-connection
connection list:
+---------------+-----------------+----------+-------------+------+
| connection_id | connection_type | protocol | source_addr | info |
+---------------+-----------------+----------+-------------+------+
```

## 10. Topic List

The topic list command is used to query the current topic status of the MQTT Broker. It provides information about the topic name, type, and other relevant details.

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

## 11. Connector Management

Connectors allow MQTT brokers to connect with external systems, enabling data exchange between different platforms.

### 11.1 Create Connector

Create a new connector.

```console
% ./bin/robust-ctl mqtt connector create --connector-name=my-connector --connector-type=kafka --config='{"bootstrap_servers":"localhost:9092","topic":"test-topic","key":"test-key"}' --topic-id=1
Created successfully!
```

### 11.2 List Connectors

List all connectors in the system.

```console
% ./bin/robust-ctl mqtt connector list --connector-name=my-connector
connector list result:
+--------------+----------------+----------------+------------------------------------------------------------------------------+----------+--------+-----------+-------------+-------------+
| cluster name | connector name | connector type | connector config                                                             | topic id | status | broker id | create time | update time |
+--------------+----------------+----------------+------------------------------------------------------------------------------+----------+--------+-----------+-------------+-------------+
| mqtt-broker  | my-connector   | Kafka          | {"bootstrap_servers":"localhost:9092","topic":"test-topic","key":"test-key"} | 1        | Idle   | 0         | 1746434176  | 1746434176  |
+--------------+----------------+----------------+------------------------------------------------------------------------------+----------+--------+-----------+-------------+-------------+
```

### 11.3 Update Connector

Update an existing connector.

```console
% ./bin/robust-ctl mqtt connector update --connector='{"cluster_name":"example","connector_name":"my-connector","connector_type":"Kafka","config":"{\"bootstrap_servers\":\"localhost:9092\",\"topic\":\"test-topic-update\",\"key\":\"test-key-update\"}","topic_id":"1","status":"Running","broker_id":null,"create_time":1710000000,"update_time":1710000000}'
Updated successfully!
```

### 11.4 Delete Connector

Delete an existing connector.

```console
% ./bin/robust-ctl mqtt connector delete --connector-name=my-connector
Deleted successfully!
```

## 12. Schema Management

Schemas define the structure and format of messages, ensuring data consistency and validation.

### 12.1 Create Schema

Create a new schema.

```console
% ./bin/robust-ctl mqtt schema create --schema-name=temperature_schema --schema-type=json --schema="{\"type\":\"object\",\"properties\":{\"temperature\":{\"type\":\"number\"},\"timestamp\":{\"type\":\"integer\"}}}" --desc="create"
Created successfully!
```

### 12.2 List Schemas

List all schemas in the system.

```console
% ./bin/robust-ctl mqtt schema list
schema list result:
cluster name: example_cluster
schema name: temperature_schema
schema type: json
schema desc: create
schema: {"type":"object","properties":{"temperature":{"type":"number"},"timestamp":{"type":"integer"}}}
```

### 12.3 Update Schema

Update an existing schema.

```console
% ./bin/robust-ctl mqtt schema update --schema-name=temperature_schema --schema="{\"type\":\"object\",\"properties\":{\"temperature\":{\"type\":\"number\"},\"timestamp\":{\"type\":\"integer\"},\"unit\":{\"type\":\"string\"}}}" --desc="update"
Updated successfully!
```

### 12.4 Delete Schema

Delete an existing schema.

```console
% ./bin/robust-ctl mqtt schema delete --schema-name=temperature_schema
Deleted successfully!
```

### 12.5 Bind Schema

Bind a schema to a topic.

```console
% ./bin/robust-ctl mqtt schema bind --schema-name=temperature_schema --resource-name=test
Created successfully!
```

### 12.6 Unbind Schema

Unbind a schema from a topic.

```console
% ./bin/robust-ctl mqtt schema unbind --schema-name=temperature_schema --resource-name=test
Deleted successfully!
```

### 12.7 List Bind Schemas

List all schema bindings.

```console
% ./bin/robust-ctl mqtt schema list-bind
bind schema list result:
cluster name: example_cluster
schema name: temperature_schema
schema type: json
schema desc: update
schema: {"type":"object","properties":{"temperature":{"type":"number"},"timestamp":{"type":"integer"}}}
```

## 13. Auto Subscribe Rules

Auto subscribe rules allow the broker to automatically subscribe clients to specific topics based on predefined rules.

### 13.1 Set Auto Subscribe Rule

Create or update an auto subscribe rule.

```console
% ./bin/robust-ctl mqtt auto-subscribe-rule set --topic=test/topic --qos=1 --no-local --retain-as-published --retained-handling=1
Created successfully!
```

### 13.2 List Auto Subscribe Rules

List all auto subscribe rules.

```console
% ./bin/robust-ctl mqtt auto-subscribe-rule list
+------------+-----+----------+---------------------+-------------------+
| topic      | qos | no_local | retain_as_published | retained_handling |
+------------+-----+----------+---------------------+-------------------+
| test/topic | 1   | true     | true                | 1                 |
+------------+-----+----------+---------------------+-------------------+
```

### 13.3 Delete Auto Subscribe Rule

Delete an existing auto subscribe rule.

```console
% ./bin/robust-ctl mqtt auto-subscribe-rule delete --topic=test/topic
Deleted successfully!
```
