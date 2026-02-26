# Modbus TCP to RTU Bridge

## Run Instructions
You can start the bridge right away. The default properties will bind the TCP server to port 5020, use /dev/ttyUSB0 as the serial port, and 9600 as the baud rate.

    # Run with defaults
    cargo run --release
    # Run with custom arguments
    cargo run --release -- --tcp-bind 0.0.0.0:5020 --serial-port /dev/ttyUSB1 --baud-rate 115200

