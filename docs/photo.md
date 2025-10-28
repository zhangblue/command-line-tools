# 照片操作

本功能主要用于对照片的操作，目前支持功能如下.
对照片的操作，目前支持的照片格式有：`["TIFF", "RAW", "HEIF", "JPEG", "WEBP", "PNG", "JPG", "HEIC"]`

- time: 获取照片的拍摄时间
- group: 按照照片的拍摄时间将照片按照`yyyy-mm-dd`的目录进行归类

```shell
操作照片
- 目前支持的照片格式：[TIFF, RAW, HEIF, JPEG, WEBP, PNG, JPG, HEIC]

Usage: rust_cmd photo <COMMAND>

Commands:
  group  照片按照拍摄时间分类
  time   得到照片的拍摄时间
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')
```

## 获取照片拍摄时间

```shell
得到照片的拍摄时间

Usage: rust_cmd photo time --photo-dir <PHOTO_DIR>

Options:
  -p, --photo-dir <PHOTO_DIR>
          照片路径

  -h, --help
          Print help (see a summary with '-h')
```

参数详解：

- `photo-dir`: 要获取的照片位置

## 按照拍摄时间归类

```shell
得到照片的拍摄日期，按照拍摄时间分类

Usage: rust_cmd photo group [OPTIONS] --scan-dir <SCAN_DIR> --output-dir <OUTPUT_DIR>

Options:
  -s, --scan-dir <SCAN_DIR>
          扫描的目录

  -o, --output-dir <OUTPUT_DIR>
          输出目录

  -a, --action <ACTION>
          分组时采用何种方式操作文件

          [default: move]
          [possible values: move, copy]

  -h, --help
          Print help (see a summary with '-h')
```

参数详解：

- `scan-dir`: 要扫描的目录
- `output-dir`: 输出的结果目录。结果目录会自动创建，并按照`yyyy-MM-dd`的目录进行归类
- `action`: 移动方式
    - `move`: 移动原始照片到输出目录(默认)
    - `copy`: 拷贝原始照片到输出目录