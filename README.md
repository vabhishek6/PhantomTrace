
# PhantomTrace
<p align="left"> <img src="https://github.com/user-attachments/assets/f06b6216-f1c3-4131-8201-8b3171669902" width="280" alt="PhantomTrace logo"> </p>


**Log Obfuscation Tool for PCI, PII, and Confidential Data**

Built for secure detection and masking of sensitive data in application logs and files.

[Quick Start](#quick-start) | [Documentation](#documentation) | [Configuration](#configuration) | [Examples](#examples)

***

## Overview

PhantomTrace is a high-performance tool for detecting and obfuscating sensitive data in text files, logs, and data streams.  
It is designed with security and compliance requirements in mind, particularly for PCI, PII, and related regulations.

### Key Features
- **Standards Compliance** – Suitable for PCI DSS, GDPR, HIPAA, and CCPA workflows.  
- **High Throughput** – Processes 50K–100K+ lines per second.  
- **Accurate Detection** – Uses advanced, tested regex patterns.  
- **Multiple Obfuscation Methods** – Phantom, Vanish, Mirror, Mask, and Tokenize.  
- **Detailed Reporting** – Generate trace and coverage reports.  
- **Full Configurability** – Define custom patterns and processing rules.  
- **Format Support** – Handles text, JSON, CSV, and trace report output.

***

## Quick Start

### Installation

**From Crates.io**
```bash
cargo install phantomtrace
```

**From Source**
```bash
git clone https://github.com/yourusername/phantomtrace
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

## Library Usage (Rust)

```rust
use phantomtrace::{phantom_text, PhantomTraceConfig, PhantomTraceProcessor};

let input = "User: john.doe@example.com, Card: 4532-1234-5678-9012";
let result = phantom_text(input)?;

// Output: "User: joh███████████@example.com, Card: ████-████-████-9012"
println!("Result: {}", result);

// Advanced usage
let config = PhantomTraceConfig::default();
let mut processor = PhantomTraceProcessor::new(config)?;
let processed = processor.phantom_text(input);

println!("Phantomed: {}", processed.phantomed_text);
println!("Events: {}", processed.phantom_events.len());
```

***

## Configuration

PhantomTrace includes default patterns for common sensitive data types such as credit card numbers, SSNs, email addresses, phone numbers, IP addresses, and API keys.  
You can extend or override these with a JSON configuration file.

Example custom rule:
```json
{
  "name": "custom_id",
  "pattern": "\\bCUST-\\d{6}\\b",
  "method": "Phantom",
  "preserve_chars": 4,
  "severity": "Medium"
}
```

***

## Performance

| Hardware Class | Throughput    | Memory Usage |
|----------------|---------------|--------------|
| Desktop/Laptop | 50K–75K lines/sec | Moderate   |

Performance depends on batch size, enabled rules, and whether additional reports are generated.

***

## Development

```bash
git clone https://github.com/yourusername/phantomtrace
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

