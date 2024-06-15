use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhantomTraceConfig {
    pub tracing: TracingConfig,
    pub processing: ProcessingConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    pub enabled: bool,
    pub rules: Vec<TraceRule>,
    pub custom_patterns: Vec<CustomPattern>,
    pub case_sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceRule {
    pub name: String,
    pub pattern: String,
    pub method: ObfuscationMethod,
    pub preserve_chars: Option<usize>,
    pub replacement: Option<String>,
    pub severity: TraceSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObfuscationMethod {
    Phantom,        // Replace with phantom characters (****)
    Vanish,         // Remove entirely
    Mirror,         // Replace with hash/token
    Mask,           // Replace with custom string
    Tokenize,       // Replace with traceable token
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceSeverity {
    Critical,   // PCI data (credit cards, etc.)
    High,       // PII data (SSN, emails, etc.)
    Medium,     // Sensitive data (IPs, names, etc.)
    Low,        // Other identifiable data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPattern {
    pub name: String,
    pub regex: String,
    pub description: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
    pub batch_size: usize,
    pub preserve_structure: bool,
    pub trace_overlaps: bool,
    pub performance_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: OutputFormat,
    pub include_trace_report: bool,
    pub log_phantom_events: bool,
    pub create_trace_map: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
    TraceReport,
}

impl Default for PhantomTraceConfig {
    fn default() -> Self {
        Self {
            tracing: TracingConfig {
                enabled: true,
                rules: default_trace_rules(),
                custom_patterns: Vec::new(),
                case_sensitive: false,
            },
            processing: ProcessingConfig {
                batch_size: 1000,
                preserve_structure: true,
                trace_overlaps: true,
                performance_mode: false,
            },
            output: OutputConfig {
                format: OutputFormat::Text,
                include_trace_report: true,
                log_phantom_events: false,
                create_trace_map: false,
            },
        }
    }
}

fn default_trace_rules() -> Vec<TraceRule> {
    vec![
        // Credit Card Numbers (Critical PCI Data)
        TraceRule {
            name: "credit_card".to_string(),
            pattern: r"\b(?:\d{4}[-\s]?){3}\d{4}\b".to_string(),
            method: ObfuscationMethod::Phantom,
            preserve_chars: Some(4),
            replacement: None,
            severity: TraceSeverity::Critical,
        },
        // Social Security Numbers (High PII)
        TraceRule {
            name: "ssn".to_string(),
            pattern: r"\b\d{3}-\d{2}-\d{4}\b".to_string(),
            method: ObfuscationMethod::Mirror,
            preserve_chars: None,
            replacement: None,
            severity: TraceSeverity::High,
        },
        // Email Addresses (High PII)
        TraceRule {
            name: "email".to_string(),
            pattern: r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b".to_string(),
            method: ObfuscationMethod::Phantom,
            preserve_chars: Some(3),
            replacement: None,
            severity: TraceSeverity::High,
        },
        // Phone Numbers (Medium PII)
        TraceRule {
            name: "phone".to_string(),
            pattern: r"\b(?:\+1[-.\s]?)?(?:\([0-9]{3}\)|[0-9]{3})[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}\b".to_string(),
            method: ObfuscationMethod::Phantom,
            preserve_chars: Some(4),
            replacement: None,
            severity: TraceSeverity::Medium,
        },
        // IP Addresses (Medium Sensitive)
        TraceRule {
            name: "ip_address".to_string(),
            pattern: r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b".to_string(),
            method: ObfuscationMethod::Mask,
            preserve_chars: None,
            replacement: Some("XXX.XXX.XXX.XXX".to_string()),
            severity: TraceSeverity::Medium,
        },
        // API Keys (Critical)
        TraceRule {
            name: "api_key".to_string(),
            pattern: r"\b[Aa][Pp][Ii][_-]?[Kk][Ee][Yy][:\s=]+[\w\-]{20,}\b".to_string(),
            method: ObfuscationMethod::Mask,
            preserve_chars: None,
            replacement: Some("[API_KEY_PHANTOMED]".to_string()),
            severity: TraceSeverity::Critical,
        },
        // AWS Access Keys
        TraceRule {
            name: "aws_access_key".to_string(),
            pattern: r"\bAKIA[0-9A-Z]{16}\b".to_string(),
            method: ObfuscationMethod::Mask,
            preserve_chars: None,
            replacement: Some("[AWS_KEY_PHANTOMED]".to_string()),
            severity: TraceSeverity::Critical,
        },
        // Generic Passwords
        TraceRule {
            name: "password".to_string(),
            pattern: r"\b[Pp][Aa][Ss][Ss][Ww][Oo][Rr][Dd][:\s=]+\S+".to_string(),
            method: ObfuscationMethod::Mask,
            preserve_chars: None,
            replacement: Some("[PASSWORD_PHANTOMED]".to_string()),
            severity: TraceSeverity::Critical,
        },
    ]
}

impl PhantomTraceConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: PhantomTraceConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn get_rules_by_severity(&self, severity: TraceSeverity) -> Vec<&TraceRule> {
        self.tracing.rules.iter()
            .filter(|rule| matches!(rule.severity, severity))
            .collect()
    }
}
