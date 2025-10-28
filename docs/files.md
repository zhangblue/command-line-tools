# 文件操作

本功能主要用于对文件的操作

```shell
文件操作

Usage: rust_cmd files <COMMAND>

Commands:
  repeat  寻找重复的文件
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')
```

## repeat 寻找重复的文件

计算文件的sha256，相同sha256的文件认为是同一个文件。并将检测到重复的结果写入到文件中。

```shell
通过计算文件的sha256，找到相同的文件

Usage: rust_cmd files repeat --scan-dir <SCAN_DIR> --output <OUTPUT>

Options:
  -s, --scan-dir <SCAN_DIR>
          扫描的目录

  -o, --output <OUTPUT>
          将结果输出到一个文件中

  -h, --help
          Print help (see a summary with '-h')
```

参数详解

- `--scan-dir`: 要扫描的目录
- `--output`: 结果输出的文件

### 输出结果

输出的结果是一个`json`格式。 `key` 为sha256结果，`value` 则为相同文件的列表

```json
{
  "39dba168b74560c0cc11c6d87b03325bc7bf3d9bd232e4fcaf36e0e88b65071c": [
    "/Downloads/test/output/2025-10-12/IMG_2239.HEIC-1.HEIC",
    "/Downloads/test/output/2025-10-12/IMG_2239.HEIC"
  ],
  "e766350ee9c2027bae0d6ec791c768fb6bd388d98bb21ee5aaca5f6a4de0f321": [
    "/Downloads/test/output/2025-08-11/IMG_2102.HEIC",
    "/Downloads/test/output/2025-08-11/IMG_2102.HEIC-1.HEIC"
  ]
}
```