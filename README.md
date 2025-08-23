
# PhantomTrace
![Phantom Banner](https://github.com/user-attachments/assets/e43f3718-1da7-4908-a72c-760418b676a5)

[![PhantomTrace CI](https://github.com/vabhishek6/PhantomTrace/actions/workflows/rust.yml/badge.svg)](https://github.com/vabhishek6/PhantomTrace/actions/workflows/rust.yml)


**Log Obfuscation Tool for PCI, PII, and Confidential Data**

Built for secure detection and masking of sensitive data in application logs and files.

[Quick Start](#quick-start) | [Documentation](#documentation) | [Configuration](#configuration) | [Examples](#examples)

***

## Overview

PhantomTrace is a high-performance data obfuscation and log preprocessing platform designed for enterprise environments 
requiring PCI DSS, GDPR, HIPAA, and regulatory compliance. It provides secure detection and obfuscation of sensitive data 
in logs, files, and real-time data streams with native integration for enterprise logging platforms.

## Features

### **Core Data Protection**
- **Pattern Recognition**: Advanced detection of credit cards, SSNs, emails, API keys, JWT tokens, database connections, and custom sensitive data
- **Multiple Obfuscation Methods**: Phantom (masking), Vanish (removal), Mirror (hashing), Mask (replacement), Tokenize (traceable tokens)
- **Severity-Based Processing**: Critical, High, Medium, Low priority handling with customizable rules
- **Comprehensive Reporting**: Detailed trace reports, event logging, coverage analytics, and processing metrics

### **Log Preprocessing & Enterprise Integration**
- **Real-Time Stream Processing**: stdin/stdout pipeline integration for live log processing
- **TCP Server Mode**: Network service for distributed log collection and processing
- **File Monitoring**: Real-time processing of log files with automatic change detection
- **Splunk Integration**: Native compatibility with Splunk Universal Forwarder and Enterprise
- **ELK Stack Support**: Elasticsearch-ready JSON output with metadata
- **Log Shipper Compatibility**: Works with Filebeat, Fluentd, Logstash, and other common shippers

### **Enterprise Operations**
- **High-Performance Processing**: Multi-threaded operation supporting 50K+ lines per second
- **Multiple Operational Modes**: Standalone, stream processor, TCP server, file monitor, health server
- **Configuration Management**: Presets for Splunk, ELK, high-performance, and custom deployments
- **Health Monitoring**: Built-in health checks, metrics collection, and graceful shutdown handling
- **Production Ready**: Signal handling, error recovery, audit logging, and daemon mode support

***

## Quick Start

### Installation

**From Crates.io**
```bash
cargo install phantomtrace
```

**From Source**
```bash
git clone https://github.com/vabhishek6/PhantomTrace
cd phantomtrace
cargo build --release
```

**Download Binary**  
Binaries are available in [Releases](https://github.com/yourusername/phantomtrace/releases).

***

### Basic Usage

```bash
# Process a file with default patterns and settings
phantomtrace -i sensitive_data.log -o cleaned_data.log

# Generate a default configuration file
phantomtrace --generate-config phantom_config.json

# Run with a custom configuration
phantomtrace -i data.txt -o clean.txt -c phantom_config.json

# Output with a trace report in JSON format
phantomtrace -i logs.txt -o clean.txt --trace-report --format json
```

***

## Examples

**Basic processing**
```bash
phantomtrace -i app.log -o clean.log
```

**With detailed reporting**
```bash
phantomtrace -i database.log -o clean.log --trace-report --log-phantoms
```

**CSV output for analysis**
```bash
phantomtrace -i audit.log -o events.csv --format csv
```

**Trace map creation**
```bash
phantomtrace -i system.log -o clean.log --create-trace-map
```

***

## Configuration

### **Built-in Patterns**
PhantomTrace includes production-ready patterns for:
- **PCI Data**: Credit cards, CVV numbers, payment tokens
- **PII Data**: SSN, email addresses, phone numbers, addresses
- **Security**: API keys, JWT tokens, AWS access keys, passwords
- **Infrastructure**: IP addresses, database connections, URLs
- **Custom**: Configurable regex patterns for domain-specific data

### **Configuration Presets**
- **`default`**: Balanced performance and security for general use
- **`splunk`**: Optimized for Splunk Universal Forwarder integration
- **`elk`**: Configured for ELK Stack (Elasticsearch/Logstash/Kibana)
- **`high-performance`**: Maximum throughput optimization for high-volume environments

### **Sample Configuration**
```
{
"tracing": {
"enabled": true,
"case_sensitive": false,
"rules": [
{
"name": "custom_api_key",
"pattern": "\\bapi[_-]key[:\\s=]+[\\w\\-]{32,}\\b",
"method": "Mask",
"replacement": "[API_KEY_REDACTED]",
"severity": "Critical"
}
]
},
"preprocessing": {
"mode": "StreamProcessor",
"splunk_integration": {
"enabled": true,
"phantom_sourcetype": "app_logs_phantomed"
}
},
"processing": {
"performance_mode": true,
"batch_size": 5000
}
}
```

---

## Performance & Scalability

### **Throughput Benchmarks**
| Configuration | Lines/Second | Memory Usage | CPU Usage |
|---------------|--------------|--------------|-----------|
| Standard      | 25K-35K      | ~50MB        | 1-2 cores |
| Performance   | 50K-75K      | ~100MB       | 2-4 cores |
| High-Volume   | 100K+        | ~200MB       | 4-8 cores |

### **Optimization Options**
```
# Maximum performance configuration
phantomtrace --performance-mode --workers 16 --buffer-size 50000

# Memory-optimized for constrained environments
phantomtrace --workers 2 --buffer-size 1000

# High-throughput stream processing
phantomtrace --stream --performance-mode --workers 8
```

---

## Production Deployment

### **Systemd Service**
```
[Unit]
Description=PhantomTrace Log Preprocessor
After=network.target

[Service]
Type=simple
User=phantom
ExecStart=/usr/local/bin/phantomtrace --tcp-server 5140 --config /etc/phantom/config.json
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

### **Kubernetes Deployment**
```
apiVersion: apps/v1
kind: Deployment
metadata:
name: phantomtrace
spec:
replicas: 3
selector:
matchLabels:
app: phantomtrace
template:
metadata:
labels:
app: phantomtrace
spec:
containers:
- name: phantomtrace
image: phantomtrace:latest
ports:
- containerPort: 5140
args: ["--tcp-server", "5140", "--splunk-mode"]
resources:
requests:
memory: "128Mi"
cpu: "100m"
limits:
memory: "512Mi"
cpu: "500m"
```

---

## Output Formats

- **Text**: Standard text output with obfuscated content for traditional log processing
- **JSON**: Structured output with metadata for system integration and APIs
- **CSV**: Event-based output for analysis, reporting, and compliance auditing
- **Trace Report**: Comprehensive processing reports with statistics and compliance data

---

## Development

```bash
git clone https://github.com/vabhishek6/PhantomTrace
cd phantomtrace
cargo build
cargo test
cargo bench
cargo clippy -- -D warnings
```

***

## Troubleshooting

- **Pattern not detected?** Verify regex syntax and escaping in the configuration file.  
- **Slow performance?** Enable `"performance_mode": true` and/or increase `batch_size`.  
- **Regex compilation errors?** Test patterns using a Rust-compatible regex tester.

***

## License

This project is licensed under an MIT-style license for non-commercial use only. Users may freely use, modify, and distribute the software for non-commercial purposes with proper credit to the original author(s).

Commercial use, including incorporation into commercial products or services, requires a separate commercial license agreement. Companies and individuals interested in commercial licensing should contact the author of this repo

Please refer to the LICENSE file for full details.

***

## Commit History Notes

Some commits are intentionally backdated to reflect earlier development milestones.  
The creation date shown on GitHub reflects when the repository was published there.

***
[⭐ Star on GitHub](https://github.com/vabhishek6/PhantomTrace) -  [🚀 Try it now](#-quick-start) -  [📖 Read the docs](#-documentation)




