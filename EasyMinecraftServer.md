---
title: EasyMinecraftServer
language_tabs:
  - shell: Shell
  - http: HTTP
  - javascript: JavaScript
  - ruby: Ruby
  - python: Python
  - php: PHP
  - java: Java
  - go: Go
toc_footers: []
includes: []
search: true
code_clipboard: true
highlight_theme: darkula
headingLevel: 2
generator: "@tarslib/widdershins v4.0.23"

---

# EasyMinecraftServer

Base URLs:

# Authentication

# 用户

## POST 登录

POST /users/login

> Body 请求参数

```json
{
  "username": "1",
  "password": "1"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

## GET 获取用户列表

GET /users

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

## POST 注册

POST /users/register

> Body 请求参数

```json
{
  "username": "1",
  "password": "1",
  "code": 20
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

## GET 查看名片

GET /

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

# 游戏相关/MC服务器

## POST 通告发布

POST /api/notice

> Body 请求参数

```json
{
  "data": "test text",
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IjEiLCJyb2xlIjoib3duZXIiLCJleHAiOjEwMDAwMDAwMDAwMDAwMDAwfQ.jOHKS_PMb7LJd7f7vt_vHH2u9togSRxV_x7A0d2PN7s"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

## DELETE 踢出玩家

DELETE /api/kickoff

> Body 请求参数

```json
{
  "data": "hehehe3274",
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IjEiLCJyb2xlIjoib3duZXIiLCJleHAiOjEwMDAwMDAwMDAwMDAwMDAwfQ.jOHKS_PMb7LJd7f7vt_vHH2u9togSRxV_x7A0d2PN7s"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

## POST 设置管理员

POST /api/op

> Body 请求参数

```json
{
  "data": "hehehe3274",
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IjEiLCJyb2xlIjoidXNlciIsImV4cCI6MTAwMDAwMDAwMDAwMDAwMDB9.NycZsFuz9kTZakkZ7gZ9SxwhNK-8cNaSFUbONSxa0xU"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

## DELETE 取消管理员

DELETE /api/deop

> Body 请求参数

```json
{
  "data": "hehehe3274",
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IjEiLCJyb2xlIjoidXNlciIsImV4cCI6MTAwMDAwMDAwMDAwMDAwMDB9.NycZsFuz9kTZakkZ7gZ9SxwhNK-8cNaSFUbONSxa0xU"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

## POST 新增用户

POST /api/add

> Body 请求参数

```json
{
  "data": "hehehe3274",
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IjEiLCJyb2xlIjoib3duZXIiLCJleHAiOjEwMDAwMDAwMDAwMDAwMDAwfQ.jOHKS_PMb7LJd7f7vt_vHH2u9togSRxV_x7A0d2PN7s"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

## DELETE 取消用户

DELETE /api/del

> Body 请求参数

```json
{
  "data": "hehehe3274",
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IjEiLCJyb2xlIjoidXNlciIsImV4cCI6MTAwMDAwMDAwMDAwMDAwMDB9.NycZsFuz9kTZakkZ7gZ9SxwhNK-8cNaSFUbONSxa0xU"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

## PUT 安全停机

PUT /api/stop

> Body 请求参数

```json
{
  "data": "",
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IjEiLCJyb2xlIjoib3duZXIiLCJleHAiOjEwMDAwMDAwMDAwMDAwMDAwfQ.jOHKS_PMb7LJd7f7vt_vHH2u9togSRxV_x7A0d2PN7s"
}
```

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|body|body|object| 否 |none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|成功|Inline|

### 返回数据结构

# 数据模型

