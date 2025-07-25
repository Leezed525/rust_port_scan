# PortScanner

一个通过tcp握手连接来检测端口的端口扫描器

## 功能

| 功能      | 描述                 | 完成情况 | 备注    |
|---------|--------------------|------|-------|
| 端口扫描    | 通过tcp握手连接来检测端口是否开放 | 基本完成 | 支持多线程 |
| 域名支持    | 通过域名，解析出来后再进行扫描    | 未完成  |       |
| 多ip同时扫描 | 一次性扫描多个ip          | 未完成  |       |

## 使用

```bash
cargo run -- -ip <ip> -start <start_port> -end <end_port> -max_concurrent <max_concurrent>
```

## 可选参数

| 参数               | 描述              | 默认值   | 是否必选 |
|------------------|-----------------|-------|------|
| --ip             | 需要扫描的ip         | 无     | ✅    |
| --begin(-b)      | 起始端口            | 80    | ❌    |
| --end(-e)        | 结束端口            | 65535 | ❌    |
| --dura(-d)       | 判定端口不连接的时间(ms)  | 2000  | ❌    |
| --max_concurrent | 并发连接数(发起连接的线程数) | 768   | ❌    |