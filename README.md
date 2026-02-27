# Modbus TCP to RTU Bridge

## Run Instructions
You can start the bridge right away. The default properties will bind the TCP server to port 5020, use /dev/ttyUSB0 as the serial port, and 9600 as the baud rate.

    # Run with defaults
    cargo run --release
    # Run with custom arguments
    cargo run --release -- --tcp-bind 0.0.0.0:5020 --serial-port /dev/ttyUSB1 --baud-rate 115200

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
