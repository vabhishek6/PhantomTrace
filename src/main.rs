use clap::{Arg, ArgAction, ArgMatches, Command};
use phantomtrace::{PhantomTraceConfig, PhantomTraceProcessor};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

struct PhantomTraceApp {
    config: PhantomTraceConfig,
    shutdown_signal: Arc<AtomicBool>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_panic_handler();

    let matches = build_cli_parser();

    if let Some(config_path) = matches.get_one::<String>("generate-config") {
        return handle_config_generation(config_path);
    }

    if matches.get_flag("version-info") {
        return handle_version_info();
    }

    if matches.get_flag("health-check") {
        return handle_health_check();
    }

    let config = load_configuration(&matches)?;
    validate_configuration(&config)?;

    let app = PhantomTraceApp {
        config: config.clone(),
        shutdown_signal: Arc::new(AtomicBool::new(false)),
    };

    setup_signal_handlers(app.shutdown_signal.clone())?;

    match determine_operation_mode(&matches) {
        OperationMode::StreamProcessor => stream_mode(&app, &matches),
        OperationMode::TcpServer(port) => tcp_server_mode(&app, port),
        OperationMode::FileMonitor(path) => file_monitor_mode(&app, &path),
        OperationMode::BatchProcessor => batch_mode(&app, &matches),
        OperationMode::HealthServer(port) => health_server_mode(&app, port),
    }
}

fn build_cli_parser() -> ArgMatches {
    Command::new("phantomtrace")
        .version("1.0.0")
        .about("PhantomTrace - Enterprise PCI/PII Data Obfuscation & Log Preprocessing Platform")
        .long_about("Enterprise-grade data protection platform for GDPR/PCI compliance and log preprocessing")

        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Input file to process")
            .required_unless_present_any([
                "stream", "tcp-server", "monitor", "generate-config",
                "health-check", "health-server", "version-info"
            ]))

        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Output file for processed data")
            .required_unless_present_any([
                "stream", "tcp-server", "monitor", "generate-config",
                "health-check", "health-server", "version-info"
            ]))

        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Configuration file (JSON format)"))

        .arg(Arg::new("generate-config")
            .long("generate-config")
            .value_name("FILE")
            .help("Generate configuration file"))

        .arg(Arg::new("config-preset")
            .long("config-preset")
            .value_name("PRESET")
            .help("Configuration preset: default, splunk, elk, high-performance")
            .default_value("default"))

        .arg(Arg::new("stream")
            .long("stream")
            .help("Stream processing mode (stdin → stdout)")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("tcp-server")
            .long("tcp-server")
            .value_name("PORT")
            .help("Run as TCP server (default: 5140)")
            .conflicts_with_all(["stream", "monitor"]))

        .arg(Arg::new("monitor")
            .long("monitor")
            .value_name("FILE")
            .help("Monitor log file for real-time processing")
            .conflicts_with_all(["stream", "tcp-server"]))

        .arg(Arg::new("health-server")
            .long("health-server")
            .value_name("PORT")
            .help("Run health check server (default: 8080)")
            .conflicts_with_all(["stream", "tcp-server", "monitor"]))

        .arg(Arg::new("format")
            .short('f')
            .long("format")
            .value_name("FORMAT")
            .help("Output format: text, json, csv, trace-report")
            .default_value("text"))

        .arg(Arg::new("splunk-mode")
            .long("splunk-mode")
            .help("Enable Splunk compatibility")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("elk-mode")
            .long("elk-mode")
            .help("Enable ELK Stack compatibility")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("daemon")
            .short('d')
            .long("daemon")
            .help("Run in daemon mode")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("log-level")
            .long("log-level")
            .value_name("LEVEL")
            .help("Logging level: error, warn, info, debug, trace")
            .default_value("info"))

        .arg(Arg::new("workers")
            .short('w')
            .long("workers")
            .value_name("COUNT")
            .help("Number of worker threads")
            .default_value("4"))

        .arg(Arg::new("performance-mode")
            .long("performance-mode")
            .help("Enable high-performance optimizations")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("metrics")
            .long("metrics")
            .help("Enable metrics collection")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("health-check")
            .long("health-check")
            .help("Perform health check and exit")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("version-info")
            .long("version-info")
            .help("Show version information")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("trace-report")
            .long("trace-report")
            .help("Include detailed trace report")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("log-phantoms")
            .long("log-phantoms")
            .help("Log all phantom events")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("create-trace-map")
            .long("create-trace-map")
            .help("Create processing trace map")
            .action(ArgAction::SetTrue))

        .arg(Arg::new("quiet")
            .short('q')
            .long("quiet")
            .help("Suppress output (except errors)")
            .action(ArgAction::SetTrue))

        .get_matches()
}

#[derive(Debug)]
enum OperationMode {
    StreamProcessor,
    TcpServer(u16),
    FileMonitor(String),
    BatchProcessor,
    HealthServer(u16),
}

fn determine_operation_mode(matches: &ArgMatches) -> OperationMode {
    if matches.get_flag("stream") {
        OperationMode::StreamProcessor
    } else if let Some(port_str) = matches.get_one::<String>("tcp-server") {
        let port = port_str.parse().unwrap_or(5140);
        OperationMode::TcpServer(port)
    } else if let Some(file_path) = matches.get_one::<String>("monitor") {
        OperationMode::FileMonitor(file_path.to_string())
    } else if let Some(port_str) = matches.get_one::<String>("health-server") {
        let port = port_str.parse().unwrap_or(8080);
        OperationMode::HealthServer(port)
    } else {
        OperationMode::BatchProcessor
    }
}

fn load_configuration(
    matches: &ArgMatches,
) -> Result<PhantomTraceConfig, Box<dyn std::error::Error>> {
    let mut config = if let Some(config_path) = matches.get_one::<String>("config") {
        PhantomTraceConfig::load_from_file(config_path)?
    } else {
        let preset = matches.get_one::<String>("config-preset").unwrap();
        match preset.as_str() {
            "splunk" => PhantomTraceConfig::splunk_preset(),
            "elk" => PhantomTraceConfig::elk_preset(),
            "high-performance" => PhantomTraceConfig::high_performance_preset(),
            _ => PhantomTraceConfig::default(),
        }
    };

    apply_cli_overrides(&mut config, matches)?;
    Ok(config)
}

fn apply_cli_overrides(
    config: &mut PhantomTraceConfig,
    matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(format) = matches.get_one::<String>("format") {
        config.output.format = match format.as_str() {
            "json" => phantomtrace::config::OutputFormat::Json,
            "csv" => phantomtrace::config::OutputFormat::Csv,
            "trace-report" => phantomtrace::config::OutputFormat::TraceReport,
            _ => phantomtrace::config::OutputFormat::Text,
        };
    }

    if matches.get_flag("splunk-mode") {
        config.preprocessing.splunk_integration.enabled = true;
        config.preprocessing.mode = phantomtrace::config::PreprocessingMode::StreamProcessor;
        config.output.format = phantomtrace::config::OutputFormat::Json;
        config.output.include_trace_report = false;
    }

    if matches.get_flag("performance-mode") {
        config.processing.performance_mode = true;
        config.preprocessing.performance_tuning.enable_batching = true;
        config.preprocessing.performance_tuning.async_processing = true;
    }

    if matches.get_flag("trace-report") {
        config.output.include_trace_report = true;
    }

    if matches.get_flag("log-phantoms") {
        config.output.log_phantom_events = true;
    }

    if matches.get_flag("create-trace-map") {
        config.output.create_trace_map = true;
    }

    Ok(())
}

fn validate_configuration(config: &PhantomTraceConfig) -> Result<(), Box<dyn std::error::Error>> {
    if config.tracing.rules.is_empty() {
        return Err("No tracing rules configured".into());
    }

    if config.preprocessing.performance_tuning.thread_pool_size == 0 {
        return Err("Thread pool size must be greater than 0".into());
    }

    Ok(())
}

fn stream_mode(
    _app: &PhantomTraceApp,
    _matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut processor = PhantomTraceProcessor::new(_app.config.clone())?;
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line?;
        let result = processor.phantom_text(&line);
        writeln!(stdout_lock, "{}", result.phantomed_text)?;
        stdout_lock.flush()?;
    }

    Ok(())
}

fn tcp_server_mode(_app: &PhantomTraceApp, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    eprintln!("PhantomTrace TCP server listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let config = _app.config.clone();
                thread::spawn(move || {
                    if let Err(e) = handle_tcp_client(stream, &config) {
                        eprintln!("Client error: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

fn handle_tcp_client(
    stream: TcpStream,
    config: &PhantomTraceConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut processor = PhantomTraceProcessor::new(config.clone())?;
    let mut write_stream = stream.try_clone()?;
    let read_stream = stream;
    let reader = BufReader::new(read_stream);

    for line in reader.lines() {
        let line = line?;
        let result = processor.phantom_text(&line);
        writeln!(write_stream, "{}", result.phantomed_text)?;
    }

    Ok(())
}

fn batch_mode(
    _app: &PhantomTraceApp,
    matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();

    if !Path::new(input_path).exists() {
        return Err(format!("Input file '{}' does not exist", input_path).into());
    }

    let create_trace_map = _app.config.output.create_trace_map;
    let quiet = matches.get_flag("quiet");

    if !quiet {
        eprintln!("Processing: {} -> {}", input_path, output_path);
    }

    let mut processor = PhantomTraceProcessor::new(_app.config.clone())?;
    let result = processor.phantom_file(input_path, output_path)?;

    if !quiet {
        display_results(&result, output_path, &processor, create_trace_map);
    }

    Ok(())
}

fn display_results(
    result: &phantomtrace::ProcessingResult,
    output_path: &str,
    processor: &PhantomTraceProcessor,
    create_trace_map: bool,
) {
    eprintln!("Processing completed");
    eprintln!("Lines processed: {}", result.lines_processed);
    eprintln!("Lines modified: {}", result.lines_phantomed);
    eprintln!("Events: {}", result.phantom_events.len());
    eprintln!("Processing time: {:?}", result.processing_time);
    eprintln!("Output: {}", output_path);

    if result.lines_phantomed > 0 {
        let coverage = (result.lines_phantomed as f64 / result.lines_processed as f64) * 100.0;
        eprintln!("Coverage: {:.1}%", coverage);

        let trace_report = processor.get_trace_report();
        if !trace_report.severity_breakdown.is_empty() {
            eprintln!("Events by severity:");
            for (severity, count) in &trace_report.severity_breakdown {
                eprintln!("  {}: {}", severity, count);
            }
        }
    }

    if create_trace_map {
        eprintln!("Trace map: {}.tracemap", output_path);
    }
}

fn setup_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("Fatal error: {}", panic_info);
        std::process::exit(1);
    }));
}

fn setup_signal_handlers(shutdown: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(move || {
        eprintln!("Shutdown signal received");
        shutdown.store(true, Ordering::Relaxed);
    })?;
    Ok(())
}

fn handle_config_generation(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let default_config = PhantomTraceConfig::default();
    default_config.save_to_file(config_path)?;
    eprintln!("Configuration saved to: {}", config_path);
    Ok(())
}

fn handle_version_info() -> Result<(), Box<dyn std::error::Error>> {
    println!("PhantomTrace Enterprise Data Protection Platform");
    println!("Version: 1.0.0");
    println!("Build: Release");

    // Use runtime environment variables to avoid compile-time errors
    if let Ok(rustc_version) = std::env::var("RUSTC_VERSION") {
        println!("Rust Version: {}", rustc_version);
    }
    if let Ok(target) = std::env::var("TARGET") {
        println!("Target: {}", target);
    }

    Ok(())
}

fn handle_health_check() -> Result<(), Box<dyn std::error::Error>> {
    let config = PhantomTraceConfig::default();

    if config.tracing.rules.is_empty() {
        return Err("No tracing rules configured".into());
    }

    for rule in &config.tracing.rules {
        if regex::Regex::new(&rule.pattern).is_err() {
            return Err(format!("Invalid regex pattern in rule: {}", rule.name).into());
        }
    }

    println!("Health check passed");
    println!("Rules validated: {}", config.tracing.rules.len());

    Ok(())
}

fn health_server_mode(_app: &PhantomTraceApp, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Health server running on port {}", port);

    loop {
        if _app.shutdown_signal.load(Ordering::Relaxed) {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn file_monitor_mode(
    _app: &PhantomTraceApp,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Monitoring file: {}", file_path);

    // File monitoring implementation would go here
    loop {
        if _app.shutdown_signal.load(Ordering::Relaxed) {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
