# 更新日志

## 20250707

### 之前的多线程请求方法

#### 方法

目前的方法是通过一次性建立若干个线程，每个线程负责探测一个端口。

```rust
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let mut ping_tasks = Vec::new();
    for port in start..end {
        let semaphore = Arc::clone(&semaphore);
        ping_tasks.push(async_ping(&ip, port, dura, semaphore));
    }
    println!("{:?}", ping_tasks.len());
    println!("Start to scan");
    let results = join_all(ping_tasks).await;
```
#### 问题

但是在目前这种方法中，一旦请求的端口数过多，
1. 会导致创建太多的线程，消耗过多的系统资源。
2. 会导致请求的端口数过多，导致请求超时。
3. 请求的端口一下子太多了，导致本地的网络连接数过多，导致请求失败。

### 改进方法

目前打算重新设计，采用任务分化的思想，
将端口分成若干个小组，每个小组的端口数不超过最大并发数，然后每个小组使用一个线程进行探测。
