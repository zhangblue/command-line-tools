# Json 格式化

本功能是将json字符串格式化美观的格式，或者将美观格式的json转成一行

```shell
json格式化

Usage: rust_cmd json [OPTIONS]

Options:
  -i, --input <INPUT>
          输入文件。无此参数使用命令行输入文本

  -o, --output <OUTPUT>
          输出文件。无此参数将控制台打印结果

  -f, --format <FORMAT>
          输出格式。Compress: 压缩在一行。Pretty: 美化

          [default: compress]
          [possible values: compress, pretty]

  -h, --help
          Print help (see a summary with '-h')
```

参数详解

- `input`: 输入文件。无此参数会触发命令行交互
- `output`: 输出文件。无此参数会在终端打印。
- `format`: 输出格式
    - `Compress`: 将json写在一行，去掉美化
    - `Pretty`: 将写在一行的json进行美化