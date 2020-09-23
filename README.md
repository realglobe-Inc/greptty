# greptty

Find paths of character devices with regexps.

## Getting Started

greptty require [Rust 1.46.0 or later](https://www.rust-lang.org/tools/install). First, install it.
We will provide executable binaries for Linux(amd64, arm) and Mac.

### Installing
Clone this repository and execute next command at the repository directory.
```shell
cargo install --path .
```

## Run examples

### Detect and read usb_co2

```sh
# macOS
./examples/detect_co2.sh /dev/cu.usbmodem <<EOF
co2 ^co2=[0-9][0-9]*$
EOF

# Linux
./examples/detect_co2.sh /dev/ttyUSB <<EOF
co2 ^co2=[0-9][0-9]*$
EOF

# Linux (another ver.)
./examples/detect_co2.sh /dev/ttyACM <<EOF
co2 ^co2=[0-9][0-9]*$
EOF
```

## License
This project is licensed under the MIT License - see the [LICENSE] file for details.
