# greptty

Find paths of character devices with regexps.

## Getting Started

greptty require [Rust 1.46.0 or later](https://www.rust-lang.org/tools/install). First, install it.
We will provide executable binaries for Linux(amd64, arm) and Mac.

### Installing
Clone this repository and execute next command at the repository directory.
```shell
cargo install
```

## Run eamples

### Detect and read usb_co2

```sh
# macOS
./examples/detect_co2.sh /dev/cu.usbmodem
# Linux
./examples/detect_co2.sh /dev/ttyUSB
./examples/detect_co2.sh /dev/ttyACM
```

## License
This project is licensed under the MIT License - see the LICENSE.md file for details.
