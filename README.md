# ETH Location Tracker

## Online Version

### Build
```
export WIFI_PASSWORD=
export WIFI_SSID=
export LOCATION_SERVICE_ENDPOINT=
export LOCATION_SERVICE_KEY=
export LOCATION_SERVICE_CLIENT_ID=
ESP_IDF_SYS_ROOT_CRATE=esp32c3-online cargo build --release
```

### Flash
```
espflash flash ./target/riscv32imc-esp-espidf/release/esp32c3-online --monitor
```

## Online Version

### Build
```
export WIFI_PASSWORD=
export WIFI_SSID=
export LOCATION_SERVICE_ENDPOINT=
export LOCATION_SERVICE_KEY=
export LOCATION_SERVICE_CLIENT_ID=
ESP_IDF_SYS_ROOT_CRATE=esp32c3-offline cargo build --release
```

### Flash
```
espflash flash ./target/riscv32imc-esp-espidf/release/esp32c3-offline --monitor
```
