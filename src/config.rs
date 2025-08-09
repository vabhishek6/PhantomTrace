use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhantomTraceConfig {
    pub tracing: TracingConfig,
    pub processing: ProcessingConfig,
    pub output: OutputConfig,
    pub preprocessing: PreprocessingConfig,
    pub monitoring: MonitoringConfig, // Added missing monitoring section
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
    Phantom,  // Replace with phantom characters (****)
    Vanish,   // Remove entirely
    Mirror,   // Replace with hash/token
    Mask,     // Replace with custom string
    Tokenize, // Replace with traceable token
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceSeverity {
    Critical, // PCI data (credit cards, etc.)
    High,     // PII data (SSN, emails, etc.)
    Medium,   // Sensitive data (IPs, names, etc.)
    Low,      // Other identifiable data
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

// New preprocessing configuration for log pipeline integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingConfig {
    pub mode: PreprocessingMode,
    pub realtime_processing: bool,
    pub tcp_server_port: Option<u16>,
    pub file_monitoring: bool,
    pub splunk_integration: SplunkConfig,
    pub elk_integration: ElkConfig,
    pub performance_tuning: PerformanceTuning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreprocessingMode {
    Standalone,      // Traditional file processing
    StreamProcessor, // Real-time stream processing (stdin/stdout)
    TcpServer,       // Network server for log agents
    FileMonitor,     // Monitor log files for changes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplunkConfig {
    pub enabled: bool,
    pub preserve_timestamp: bool,
    pub preserve_source: bool,
    pub add_phantom_metadata: bool,
    pub phantom_sourcetype: String,
    pub index: Option<String>,
    pub host_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElkConfig {
    pub enabled: bool,
    pub add_phantom_fields: bool,
    pub phantom_index_pattern: String,
    pub preserve_original_timestamp: bool,
    pub add_processing_metadata: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTuning {
    pub thread_pool_size: usize,
    pub buffer_size: usize,
    pub flush_interval_ms: u64,
    pub enable_batching: bool,
    pub async_processing: bool,
    pub memory_limit_mb: usize,
}

// Added missing monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub metrics_interval: Duration,
    pub health_check_enabled: bool,
    pub audit_logging: bool,
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
            preprocessing: PreprocessingConfig::default(),
            monitoring: MonitoringConfig::default(),
        }
    }
}

impl Default for PreprocessingConfig {
    fn default() -> Self {
        Self {
            mode: PreprocessingMode::Standalone,
            realtime_processing: false,
            tcp_server_port: Some(5140),
            file_monitoring: false,
            splunk_integration: SplunkConfig {
                enabled: false,
                preserve_timestamp: true,
                preserve_source: true,
                add_phantom_metadata: true,
                phantom_sourcetype: "phantomed_logs".to_string(),
                index: Some("phantom".to_string()),
                host_field: Some("phantom_host".to_string()),
            },
            elk_integration: ElkConfig {
                enabled: false,
                add_phantom_fields: true,
                phantom_index_pattern: "phantomed-logs-*".to_string(),
                preserve_original_timestamp: true,
                add_processing_metadata: true,
            },
            performance_tuning: PerformanceTuning {
                thread_pool_size: 4,
                buffer_size: 10000,
                flush_interval_ms: 100,
                enable_batching: true,
                async_processing: false,
                memory_limit_mb: 512,
            },
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: false,
            metrics_interval: Duration::from_secs(60),
            health_check_enabled: true,
            audit_logging: false,
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
            pattern: r"\b(?:\+1[-.\s]?)?(?:\([0-9]{3}\)|[0-9]{3})[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}\b"
                .to_string(),
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

    pub fn get_rules_by_severity(&self, _severity: TraceSeverity) -> Vec<&TraceRule> {
        self.tracing
            .rules
            .iter()
            .filter(|rule| matches!(&rule.severity, _severity))
            .collect()
    }

    // Configuration presets for different use cases
    pub fn splunk_preset() -> Self {
        let mut config = Self::default();
        config.preprocessing.mode = PreprocessingMode::StreamProcessor;
        config.preprocessing.splunk_integration.enabled = true;
        config.output.format = OutputFormat::Json;
        config.output.include_trace_report = false;
        config.preprocessing.realtime_processing = true;
        config
    }

    pub fn elk_preset() -> Self {
        let mut config = Self::default();
        config.preprocessing.mode = PreprocessingMode::StreamProcessor;
        config.preprocessing.elk_integration.enabled = true;
        config.output.format = OutputFormat::Json;
        config.output.include_trace_report = false;
        config.preprocessing.realtime_processing = true;
        config
    }

    pub fn high_performance_preset() -> Self {
        let mut config = Self::default();
        config.processing.performance_mode = true;
        config.processing.batch_size = 5000;
        config.preprocessing.performance_tuning.enable_batching = true;
        config.preprocessing.performance_tuning.async_processing = true;
        config.preprocessing.performance_tuning.thread_pool_size = 8;
        config
    }

    // Added missing strict_pci_preset method
    pub fn strict_pci_preset() -> Self {
        let mut config = Self::default();
        config.processing.performance_mode = false; // Prioritize compliance over speed
        config.output.log_phantom_events = true;
        config.output.create_trace_map = true;
        config.monitoring.audit_logging = true;
        config.monitoring.metrics_enabled = true;

        // Add more aggressive PCI patterns
        config.tracing.rules.extend(vec![
            TraceRule {
                name: "cvv".to_string(),
                pattern: r"\b[Cc][Vv][Vv]?[:\s=]*\d{3,4}\b".to_string(),
                method: ObfuscationMethod::Vanish,
                preserve_chars: None,
                replacement: None,
                severity: TraceSeverity::Critical,
            },
            TraceRule {
                name: "bank_account".to_string(),
                pattern: r"\b\d{8,17}\b".to_string(),
                method: ObfuscationMethod::Mirror,
                preserve_chars: None,
                replacement: None,
                severity: TraceSeverity::Critical,
            },
        ]);
        config
    }
}
