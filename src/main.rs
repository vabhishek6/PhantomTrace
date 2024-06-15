use clap::{Arg, Command};
use phantomtrace::{PhantomTraceConfig, PhantomTraceProcessor};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("phantomtrace")
        .version("1.0.0")
        .about("👻 PhantomTrace - Advanced PCI/PII Data Obfuscation Tool")
        .long_about("PhantomTrace makes sensitive data disappear like a phantom, leaving no trace behind.\nPerfect for PCI DSS compliance, GDPR requirements, and data privacy protection.")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Input file to process")
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Output file for phantomed data")
            .required(true))
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Configuration file (JSON format)")
            .required(false))
        .arg(Arg::new("generate-config")
            .long("generate-config")
            .value_name("FILE")
            .help("Generate a default configuration file")
            .required(false))
        .arg(Arg::new("format")
            .short('f')
            .long("format")
            .value_name("FORMAT")
            .help("Output format: text, json, csv, trace-report")
            .default_value("text"))
        .arg(Arg::new("trace-report")
            .long("trace-report")
            .help("Include detailed trace report")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("log-phantoms")
            .long("log-phantoms")
            .help("Log phantom events")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("create-trace-map")
            .long("create-trace-map")
            .help("Create trace map file")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    // Handle config generation
    if let Some(config_path) = matches.get_one::<String>("generate-config") {
        let default_config = PhantomTraceConfig::default();
        default_config.save_to_file(config_path)?;
        println!("👻 Default PhantomTrace configuration saved to: {}", config_path);
        println!("✨ Edit this file to customize your phantom rules!");
        return Ok(());
    }

    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();
    
    // Validate input file exists
    if !Path::new(input_path).exists() {
        eprintln!("❌ Error: Input file '{}' does not exist", input_path);
        std::process::exit(1);
    }

    // Load configuration
    let mut config = if let Some(config_path) = matches.get_one::<String>("config") {
        println!("📋 Loading custom configuration from: {}", config_path);
        PhantomTraceConfig::load_from_file(config_path)?
    } else {
        println!("📋 Using default PhantomTrace configuration");
        PhantomTraceConfig::default()
    };

    // Override config with CLI options
    if let Some(format) = matches.get_one::<String>("format") {
        config.output.format = match format.as_str() {
            "json" => phantomtrace::config::OutputFormat::Json,
            "csv" => phantomtrace::config::OutputFormat::Csv,
            "trace-report" => phantomtrace::config::OutputFormat::TraceReport,
            _ => phantomtrace::config::OutputFormat::Text,
        };
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

    // Create processor and process file
    println!("\n👻 PhantomTrace v1.0.0 - Making sensitive data vanish...");
    println!("═══════════════════════════════════════════════════════════");
    println!("📁 Input file: {}", input_path);
    println!("📤 Output format: {:?}", config.output.format);
    println!("🔍 Active trace rules: {}", config.tracing.rules.len());
    
    let mut processor = PhantomTraceProcessor::new(config)?;
    let result = processor.phantom_file(input_path, output_path)?;

    // Display results with phantom-themed output
    println!("\n✅ Phantoming completed successfully!");
    println!("═══════════════════════════════════════");
    println!("📊 Lines processed: {}", result.lines_processed);
    println!("👻 Lines phantomed: {}", result.lines_phantomed);
    println!("🎯 Phantom events: {}", result.phantom_events.len());
    println!("⏱️  Processing time: {:?}", result.processing_time);
    println!("📁 Output saved to: {}", output_path);

    if result.lines_phantomed > 0 {
        let phantom_rate = (result.lines_phantomed as f64 / result.lines_processed as f64) * 100.0;
        println!("👁️  Phantom coverage: {:.1}%", phantom_rate);
        
        // Show severity breakdown
        let trace_report = processor.get_trace_report();
        if !trace_report.severity_breakdown.is_empty() {
            println!("\n🚨 Phantom Events by Severity:");
            for (severity, count) in &trace_report.severity_breakdown {
                let emoji = match severity.as_str() {
                    "Critical" => "🔴",
                    "High" => "🟠", 
                    "Medium" => "🟡",
                    "Low" => "🟢",
                    _ => "⚪",
                };
                println!("   {} {}: {}", emoji, severity, count);
            }
        }
        
        println!("\n🎉 Your sensitive data has been successfully phantomed!");
        println!("💡 No trace of the original data remains in the output.");
    } else {
        println!("ℹ️  No sensitive data patterns detected in the input.");
    }

    if config.output.create_trace_map {
        println!("🗺️  Trace map created: {}.tracemap", output_path);
    }

    Ok(())
}
