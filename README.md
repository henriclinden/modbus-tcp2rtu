[![Rust](https://github.com/henriclinden/modbus-tcp2rtu/actions/workflows/rust.yml/badge.svg)](https://github.com/henriclinden/modbus-tcp2rtu/actions/workflows/rust.yml)

# Modbus TCP to RTU Bridge

This gateway acts as a transparent middleware that allows Modbus TCP Clients (Masters) to communicate with Modbus RTU Serverss (Slaves) over a network. It handles the translation between the two protocols by reframing the Protocol Data Unit (PDU) in real-time.

```text
       MODBUS TCP                                             MODBUS RTU
    +--------------+              GATEWAY                 +---------------+
    |              |         +---------------+            |               |
    |  TCP CLIENT  | ------> |  [MBAP PDU]   | ---------> |   RTU SERVER  |
    |   (Master)   |   TCP   |       |       |   RS485    |   (Device)    |
    |              |         |   (Strip)     |   Serial   |               |
    +--------------+         |       v       |            +---------------+
                             |   [ PDU ]     | 
                             |       |       | 
                             |    (Wrap)     | 
                             |       v       | 
                             |  [ID PDU CRC] | 
                             +---------------+
```

The gateway listens for incoming TCP connections. When a packet is received, it performs a "strip-and-wrap" process to convert the message format:

- De-encapsulation: It strips the MBAP Header (7 bytes) from the incoming Modbus TCP frame.
- Identification: It extracts the Unit ID from the MBAP header to use as the RTU Slave Address.
- Reframing: It appends a 16-bit CRC (Cyclic Redundancy Check) to the end of the PDU.
- Transmission: The resulting RTU frame is sent over the serial line to the physical hardware.

## Build
The bridge is written in Rust and compiled using Cargo.

    cargo build

This will build the bridge and the two test applications.

## Run Instructions
You can start the bridge right away. The default properties will bind the TCP server to port 5020, use /dev/ttyUSB0 as the serial port, and 115200 as the baud rate.

    # Run with defaults
    cargo run --release --bin modbus-tcp2rtu
    # Run with custom arguments
    cargo run --release --bin modbus-tcp2rtu -- --tcp-bind 0.0.0.0:5020 --serial-port /dev/ttyUSB0 --baud-rate 115200

To get detailed information when running, change the log level using the RUST_LOG environment variable.

    RUST_LOG=debug cargo run --bin modbus-tcp2rtu

## Tests

A modbus RTU test server is in test_server.ps and a TCP test client is in test_client.rs. These can be used to verify that the modbus TCP-RTU bridge is working. Please note that settings, addresses, serial ports and others are all hardcoded in the test programs.

### Create a PTY pair:

    socat -d -d PTY,link=/tmp/ttyV0,raw,echo=0 PTY,link=/tmp/ttyV1,raw,echo=0

Take note on the pty pair numbering.

### Start the gateway

    RUST_LOG=debug cargo run --bin modbus-tcp2rtu -- -s /dev/pts/3

### Start the RTU server

    cargo run --bin test_server

### Start the TCP client

    cargo run --bin test_client
