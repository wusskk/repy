# Repy

[English]|[[Chinese](README_CN.md)]

This is a library for generating an embedded Python environment.

## Usage

### 1. Initialization
```bash
repy init --version $(python_version)
# python_version should be in the form x.y.z, for example, 3.12.6
```

### 2. Download pip packages
```bash
# Download packages
repy install <package1> <package2> ...

# Specify mirror URL
repy install <package1> --proxy $(url)
```