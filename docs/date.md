# 时间转换

功能描述：将日期转成时间戳 或 将时间戳转成日期

```shell
对时间进行处理

Usage: rust_cmd date <COMMAND>

Commands:
  to_date       将时间戳转成日期
  to_timestamp  将日期转成时间戳
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')
```

## 将日期转成时间戳

```shell
将日期转成时间戳

Usage: rust_cmd date to_timestamp [OPTIONS] --input <INPUT> --format <FORMAT> --unit <UNIT>

Options:
  -i, --input <INPUT>
          日期

  -f, --format <FORMAT>
          日期格式

          [possible values: yyyy-MM-dd, yyyy-MM-dd_HH, yyyy-MM-dd_HH:mm, yyyy-MM-dd_HH:mm::ss, yyyy-MM-dd_HH:mm::ss.SSS]

  -t, --timezone <TIMEZONE>
          时区。负数表示西区，正数表示东区，0表示UTC。默认使用当前时区

  -u, --unit <UNIT>
          输出时间戳单位

          [possible values: second, millisecond]

  -h, --help
          Print help (see a summary with '-h')
```

参数详解：

- `input`: 输入的日期
- `format`: 输入日期的格式。必须与输入的日期格式相同
    - 可选：`yyyy-MM-dd`, `yyyy-MM-dd_HH`, `yyyy-MM-dd_HH:mm`, `yyyy-MM-dd_HH:mm::ss`, `yyyy-MM-dd_HH:mm::ss.SSS`
- `timezone`: 输入日期的时区
- `unit`: 输出时间戳单位
    - 可选：`second`(秒)，`millisecond`(毫秒)

## 将时间戳转成日期格式

```shell
将时间戳转成日期

Usage: rust_cmd date to_date [OPTIONS]

Options:
  -i, --input <INPUT>
          时间戳。默认使用now

  -u, --unit <UNIT>
          输入时间戳的单位

          [default: millisecond]
          [possible values: second, millisecond]

  -t, --timezone <TIMEZONE>
          时区。负数表示西区，正数表示东区，0表示UTC。默认使用当前时区

  -h, --help
          Print help (see a summary with '-h')
```

- `input`: 输入的时间戳
- `unit`: 输入的时间戳是什么单位
    - 可选：`second`(秒)，`millisecond`(毫秒)
- `timezone`: 输出日期的时区