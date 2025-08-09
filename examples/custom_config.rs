use phantomtrace::{PhantomTraceConfig, PhantomTraceProcessor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from a file
    let config = PhantomTraceConfig::load_from_file("phantom_config.json")?;

    let mut processor = PhantomTraceProcessor::new(config)?;

    let input = "SSN: 123-45-6789, Email: test.user@company.org";
    let result = processor.phantom_text(input);

    println!("Processed Text:\n{}", result.phantomed_text);
    println!("Total Phantom Events: {}", result.phantom_events.len());

    for event in result.phantom_events {
        println!(
            "[{}] {} → {} (severity: {:?})",
            event.rule_name, event.original_value, event.phantom_value, event.severity
        );
    }

    Ok(())
}
