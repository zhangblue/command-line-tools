# 命令行工具

使用rust开发的命令行工具，将一些常用的命令集成到一起。

## base64 编解码

```shell
对文本或文件进行 base64 编解码

Usage: rust_cmd base64 <COMMAND>

Commands:
  encode  base64 编码
  decode  base64 解码
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')
```

### 编码

```shell
base64 编码

Usage: rust_cmd base64 encode [OPTIONS]

Options:
  -i, --input <INPUT>
          需要编码的文件路径，如果不填则手动输入文本

          [default: -]

  -f, --format <FORMAT>
          [default: standard]
          [possible values: standard, url-safe]

  -h, --help
          Print help (see a summary with '-h')
```

参数详解

- `input`: 可以输入一个文件路径，会对文件整体进行base64编码。如果不使用此参数，会触发 **命令行交互** ，用户手动数据需要编码的文本内容
- `format`: 编码方式。可选参数[`standard`,`url-safe`]
    - `standard`: 标准模式。编码后的结果可能会包含`&`等符号，无法使用在url中
    - `url-safe`：url安全模式。编码后的结果不会出现`&`等符号。

### 解码

```shell
base64 解码

Usage: rust_cmd base64 decode [OPTIONS]

Options:
  -i, --input <INPUT>
          需要解码的文件路径，如不填则手动输入文本

          [default: -]

  -f, --format <FORMAT>
          Standard: 标准模式。UrlSafe: url安全模式

          [default: standard]
          [possible values: standard, url-safe]

  -h, --help
          Print help (see a summary with '-h')
```

参数详解：同编码。解码与编码的`--format`参数必须相同，否则会解码失败。