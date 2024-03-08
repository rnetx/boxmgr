# Box-Manager (boxmgr)

> **一个 sing-box 管理器**

## 如何使用

```
1. 编辑 config.json

{
  "log_level": "info",
  "secret": "xxx", // 你的登录密钥
  "listen": "0.0.0.0:9077", // 监听地址
  "data_dir": "/etc/boxmgr", // 数据目录
  "temp_dir": "/tmp" // 临时文件目录
}

2. 运行

> ./boxmgr -c <你的配置文件路径> run

*3. (仅 Windows) 安装为 Windows 服务

> ./boxmgr install-service [--binary-path <你的 boxmgr 二进制文件路径>] [--config-path <你的配置文件路径>] [--auto-start]

(其中 auto-start 为是否开机自动启动)

*4. (仅 Windows) 卸载 Windows 服务

> ./boxmgr uninstall-service

*5. (仅 Windows) 作为服务启动

> ./boxmgr start-service

*6. (仅 Windows) 停止服务

> ./boxmgr stop-service

*7. (仅 Windows) 获取服务运行状态

> ./boxmgr get-service-status
```

## 如何构建

```
准备:

1. Node.js 环境
2. Rust 环境
```

```
1. 构建界面

> cd ui
> pnpm install
> pnpm run build

2. 构建二进制文件

> cargo build --release
```

## 开源协议

此项目使用 GPLv3 协议开源
