# ETH Location Tracker

## Offline Version

### Build
```
cargo build -p esp32c3-offline --release
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
cargo build -p esp32c3-online --release
```

### Flash
```
espflash flash ./target/riscv32imc-esp-espidf/release/esp32c3-offline --monitor
```
$
