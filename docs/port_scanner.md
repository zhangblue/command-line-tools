# 端口扫描器

端口扫描器 (目前还是单线程，速度较慢，后续会优化成多线程)

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