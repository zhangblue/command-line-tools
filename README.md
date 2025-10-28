# 命令行工具

## 项目简介

使用rust实现的命令行工具，将一些平时需要使用到的功能使用Rust实现并集成在了一起。同时也借此项目学习Rust

## 目前支持的功能

```shell
rust编写的命令行工具

Usage: rust_cmd <COMMAND>

Commands:
  base64        对文本或者文件进行 base64 编解码
  date          对时间进行处理
  json          json进行格式化
  photo         照片操作
  files         文件操作
  port_scanner  端口扫描器
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

- [base64编解码](./docs/base64.md)：用于对文件或输入的文本进行`base64`的编解码。
- [json格式化](./docs/json_format.md): 用于对json文本的格式化
- [日期时间戳转换](docs/date.md): 用于日期与时间戳的互相转换
- [照片操作](./docs/photo.md): 用于对照片进行操作
- [文件操作](./docs/files.md): 用于文件操作
- [端口扫描器](./docs/port_scanner.md): 端口扫描器