# 一个简单的IOT数据收集和展示中心

## 收集

通过订阅mqtt topic，将数据写入数据库。目前仅支持sqlite。mqtt数据体：

```json
{
	"device_id": "DEVICE_ID",
	"message_type": "TYPE",
	"timestamp": 1529907482,
	"data": {
		"key": "value"
	}
}
```

其中timestamp是消息时间戳，既UNIX时间戳；data字段包含每种类型自定义数据，以json格式直接存入sqlite的text字段。
读取时可以通过sqlite的json函数进行解析。


数据库结构：
```sql
CREATE TABLE IF NOT EXISTS iot_data(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device TEXT NOT NULL,
    type TEXT NOT NULL,
    time INTEGER NOT NULL,
    data TEXT NOT NULL
)
```

## 展示
（待续。。。）