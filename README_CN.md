# Repy

[[English](README.md)]|[Chinese]

这是一个用于生成嵌入式Python环境的库

## 使用方法

### 1. 初始化
``` bash
repy init --version $(python_verison) 
# python_verison 形如 x.y.z， 例如3.12.6
```

### 2.下载pip包
```bash
# 下载包
repy install <package1> <package2> ...

# 指定镜像url
repy install <package1> --proxy $(url)
```

