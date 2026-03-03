port_sniffer

A fast asynchronous TCP port scanner written in Rust.

Features

Scan custom port ranges

Supports IP addresses and domain names

Async scanning using Tokio

Configurable start and end ports

Lightweight and minimal dependencies

Installation
Build locally
cargo build --release

Binary will be available at:

target/release/port_sniffer
Install globally (optional)
cargo install --path .

Make sure ~/.cargo/bin is in your PATH.

Usage

Show help:

port_sniffer --help

Scan localhost with default settings:

port_sniffer

Scan a specific IP:

port_sniffer -a 192.168.1.1

Scan a domain:

port_sniffer -a google.com

Scan a custom port range:

port_sniffer -a google.com -s 20 -e 100
Options

-a, --address <ADDRESS> Target IP or domain (default: 127.0.0.1)

-s, --start-port <PORT> Start port (default: 1)

-e, --end-port <PORT> End port (default: 65535)

Example Output
Resolved google.com to 142.250.x.x

Open Ports:
80 is open
443 is open
Disclaimer

Use this tool only on systems you own or have permission to test.

License

MIT