# 端口扫描器

端口扫描器.

端口扫描核心逻辑为对每一个端口创建连接，查看是否可以连接上，所以此操作是一个网络IO的频繁操作，此场景适合tokio的异步并发逻辑。

```shell
端口扫描器

Usage: rust_cmd port_scanner [OPTIONS] --addr <ADDR>

Options:
  -a, --addr <ADDR>
          要扫描的地址

  -s, --port-start <PORT_START>
          扫描起始端口

          [default: 1]

  -e, --port-end <PORT_END>
          扫描结束端口

          [default: 1024]

  -h, --help
          Print help (see a summary with '-h')
```