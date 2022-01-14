# sys-info

一个轻量级的用于获取系统信息的 Rust 应用。

## 使用

支持 HTTP 接口和回调通知两种使用方式。

### 1、CURL/HTTP GET
``` shell
curl -X GET http://localhost:8080/sys-info -H "accept: application/json"
```
Response body
``` json
{
  "code": 0,
  "msg": "Success",
  "data": {
    "platform": {
      "system": "Darwin",
      "release": "20.2.0",
      "hostname": "dev-tang的MacBook",
      "version": "Darwin Kernel Version 20.2.0: Wed Dec  2 20:39:59 PST 2020; root:xnu-7195.60.75~1/RELEASE_X86_64",
      "arch": "x86_64"
    },
    "net": [
      {
        "ip": "192.168.124.14"
      }
    ],
    "memory": {
      "total": 17179,
      "available": 11936,
      "free": 5313
    },
    "disk": [
      {
        "total": 488245288,
        "used": 14692684,
        "free": 157585996,
        "file_system": "apfs",
        "mount_point": "/"
      },
      {
        "total": 488245288,
        "used": 20,
        "free": 157585996,
        "file_system": "apfs",
        "mount_point": "/System/Volumes/VM"
      },
      {
        "total": 488245288,
        "used": 286576,
        "free": 157585996,
        "file_system": "apfs",
        "mount_point": "/System/Volumes/Preboot"
      },
      {
        "total": 488245288,
        "used": 368,
        "free": 157585996,
        "file_system": "apfs",
        "mount_point": "/System/Volumes/Update"
      },
      {
        "total": 488245288,
        "used": 314925472,
        "free": 157585996,
        "file_system": "apfs",
        "mount_point": "/System/Volumes/Data"
      }
    ],
    "cpu": {
      "count": 12,
      "usage": 47.97817
    }
  }
}
```

### 2、回调通知
支持配置回调地址，定时上报数据到指定的 URL。具体参数配置说明请看 [config.toml](config.toml) 文件。


## 主要特性：  
- [x] 系统平台信息  
- [x] IP  
- [x] 内存  
- [x] 磁盘  
- [x] CPU

## 支持的平台
- [x] Windows  
- [x] Linux  
- [x] MacOS  
- [x] 银河麒麟 (Phytium,FT-1500A)  
