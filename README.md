# Box-Manager (boxmgr)

> **a sing-box manager**

## How to use

```
1. edit config.json

{
  "log_level": "info",
  "secret": "xxx", // Your secret
  "listen": "0.0.0.0:9077", // Listen Address
  "data_dir": "/etc/boxmgr", // Data Directory
  "temp_dir": "/tmp" // Temp Directory
}

2. run boxmgr

> ./boxmgr -c <your config path> run

*3. (Just Windows) Install As Service

> ./boxmgr install-service [--binary-path <your boxmgr binary path>] [--config-path <your config path>] [--auto-start]

(auto-start : Start at boot)

*4. (Just Windows) Uninstall Service

> ./boxmgr uninstall-service

*5. (Just Windows) Start Service

> ./boxmgr start-service

*6. (Just Windows) Stop Service

> ./boxmgr stop-service

*7. (Just Windows) Get Service Status

> ./boxmgr get-service-status
```

## How to build

```
Prepare:

1. Node.js Environment
2. Rust Environment
```

```
1. build ui

> cd ui
> pnpm install
> pnpm run build

2. build boxmgr

> cargo build --release
```

## License

This project adopts GPLv3 open source.
