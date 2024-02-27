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

> ./boxmgr -c ./config.json
```

## How to build

```
1. build ui

> cd ui
> pnpm run build

2. build boxmgr

> cargo build --release
```

##
