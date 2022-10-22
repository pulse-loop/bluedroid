# Bluedroid Rust wrapper

This is a Rust wrapper for the Bluedroid Bluetooth stack for ESP32.

## Features

- [x] GATT server
  - [x] Advertisement
    - [x] Custom name
    - [x] Custom appearance
  - [x] Multiple applications
  - [x] Services
    - [x] Declaration
    - [x] Advertisement
  - [x] Characteristics
    - [x] Declaration
    - [x] Broadcast
    - [x] Read
      - [x] Static
      - [x] Dynamic
      - [ ] Long
    - [ ] Write
      - [ ] With response
      - [x] Without response
      - [ ] Long
    - [ ] Notify
    - [ ] Indicate
  - [x] Descriptors
    - [x] Declaration
    - [x] Read
  - [ ] Encryption
- [ ] GATT client
  > There are currently no plans to implement the GATT client API.
  > Contributions are welcome.
- [ ] BR/EDR
  > There are currently no plans to implement the Bluetooth Classic API.
  > Contributions are welcome.
