//! # PhantomTrace 👻
//! 
//! PhantomTrace is a powerful PCI/PII data obfuscation library that makes sensitive data 
//! disappear like a phantom, leaving no trace behind.
//! 
//! ## Features
//! 
//! - **Advanced Pattern Recognition**: Detects credit cards, SSNs, emails, API keys, and more
//! - **Multiple Phantom Methods**: Phantom, Vanish, Mirror, Mask, and Tokenize
//! - **Severity-Based Processing**: Critical, High, Medium, Low severity levels
//! - **Comprehensive Reporting**: Detailed trace reports and statistics
//! - **High Performance**: Process thousands of lines per second
//! 
//! ## Quick Start
//! 
//! ```
//! use phantomtrace::{phantom_text, PhantomTraceConfig, PhantomTraceProcessor};
//! 
//! // Simple phantoming with defaults
//! let input = "User email: john.doe@example.com, SSN: 123-45-6789";
//! let phantomed = phantom_text(input)?;
//! println!("Phantomed: {}", phantomed);
//! 
//! // Advanced usage with custom config
//! let config = PhantomTraceConfig::default();
//! let mut processor = PhantomTraceProcessor::new(config)?;
//! let result = processor.phantom_text(input);
//! ```

pub mod config;
pub mod tracer;
pub mod processor;

// Re-export main types for easy access
pub use config::{
    PhantomTraceConfig, TracingConfig, TraceRule, ObfuscationMethod, 
    TraceSeverity, OutputFormat, ProcessingConfig, OutputConfig
};
pub use tracer::{PhantomTracer, PhantomEvent, TraceReport, TraceStats};
pub use processor::{PhantomTraceProcessor, ProcessingResult, ProcessingStatsOutput};

/// Simple function to phantom text with default patterns
pub fn phantom_text(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = PhantomTraceConfig::default();
    let mut processor = PhantomTraceProcessor::new(config)?;
    let result = processor.phantom_text(input);
    Ok(result.phantomed_text)
}

/// Create a processor with custom configuration
pub fn create_phantom_processor(config: PhantomTraceConfig) -> Result<PhantomTraceProcessor, Box<dyn std::error::Error>> {
    PhantomTraceProcessor::new(config)
}

/// Quick phantom for a single string with specific method
pub fn phantom_value(value: &str, method: ObfuscationMethod) -> String {
    match method {
        ObfuscationMethod::Phantom => {
            let len = value.len();
            if len <= 4 {
                "█".repeat(len)
            } else {
                format!("{}█████{}", &value[..2], &value[len-2..])
            }
        },
        ObfuscationMethod::Mirror => {
            format!("PHANTOM_{:08X}", simple_hash(value))
        },
        ObfuscationMethod::Mask => "[PHANTOMED]".to_string(),
        ObfuscationMethod::Vanish => String::new(),
        ObfuscationMethod::Tokenize => {
            format!("PHANTOM_TOKEN_{:08X}", simple_hash(value))
        },
    }
}

fn simple_hash(input: &str) -> u32 {
    let mut hash = 2166136261u32;
    for byte in input.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(16777619);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phantom_credit_card() {
        let input = "Payment with card: 4532 1234 5678 9012";
        let result = phantom_text(input).unwrap();
        
        assert!(!result.contains("4532 1234 5678 9012"));
        assert!(result.contains("█") || result.contains("PHANTOM"));
    }

    #[test]
    fn test_phantom_email() {
        let input = "Contact: user@example.com for support";
        let result = phantom_text(input).unwrap();
        
        assert!(!result.contains("user@example.com"));
        assert!(result.contains("█") || result.contains("@"));
    }

    #[test]
    fn test_phantom_ssn() {
        let input = "SSN: 123-45-6789";
        let result = phantom_text(input).unwrap();
        
        assert!(!result.contains("123-45-6789"));
        assert!(result.contains("PHANTOM_"));
    }

    #[test]
    fn test_multiple_phantoms() {
        let input = "User: john@test.com, Card: 4111111111111111, SSN: 555-44-3333";
        let result = phantom_text(input).unwrap();
        
        assert!(!result.contains("john@test.com"));
        assert!(!result.contains("4111111111111111"));
        assert!(!result.contains("555-44-3333"));
    }

    #[test]
    fn test_phantom_value_direct() {
        assert_eq!(phantom_value("test", ObfuscationMethod::Vanish), "");
        assert_eq!(phantom_value("test", ObfuscationMethod::Mask), "[PHANTOMED]");
        assert!(phantom_value("test", ObfuscationMethod::Phantom).contains("█"));
    }
}
