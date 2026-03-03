# 🔍 port_sniffer

> A blazing-fast, asynchronous TCP port scanner written in Rust — lightweight, minimal, and built for speed.

---

## ✨ Features

- ⚡ **Async scanning** powered by [Tokio](https://tokio.rs/) for maximum performance
- 🌐 **Domain & IP support** — scan hostnames or raw IP addresses
- 🎯 **Custom port ranges** — scan exactly what you need
- 🪶 **Minimal dependencies** — stays lean and fast
- 🔧 **Simple CLI** — intuitive flags, zero config required

---

## 📦 Installation

### Build from Source

```bash
cargo build --release
```

The compiled binary will be available at:

```
target/release/port_sniffer
```

### Install Globally (optional)

```bash
cargo install --path .
```

> 💡 Make sure `~/.cargo/bin` is added to your `PATH` to use `port_sniffer` from anywhere.

---

## 🚀 Usage

### Show Help

```bash
port_sniffer --help
```

### Scan localhost (default settings)

```bash
port_sniffer
```

### Scan a specific IP address

```bash
port_sniffer -a 192.168.1.1
```

### Scan a domain name

```bash
port_sniffer -a google.com
```

### Scan a custom port range

```bash
port_sniffer -a google.com -s 20 -e 100
```

---

## ⚙️ Options

| Flag | Long Form | Description | Default |
|------|-----------|-------------|---------|
| `-a` | `--address` | Target IP address or domain name | `127.0.0.1` |
| `-s` | `--start-port` | First port in the scan range | `1` |
| `-e` | `--end-port` | Last port in the scan range | `65535` |

---

## 📋 Example Output

```
Resolved google.com to 142.250.x.x

Open Ports:
  80  is open
  443 is open
```

---

## ⚠️ Disclaimer

> **Use responsibly.** This tool is intended for use **only** on systems you own or have **explicit permission** to test. Unauthorized port scanning may be illegal in your jurisdiction.

---

## 📄 License

This project is licensed under the **MIT License** — free to use, modify, and distribute.