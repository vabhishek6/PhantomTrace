use phantomtrace::{PhantomTraceConfig, PhantomTraceProcessor};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = "sample_logs.txt";
    let output_path = "phantomed_logs.txt";

    if !Path::new(input_path).exists() {
        println!("❌ Input file '{}' not found!", input_path);
        return Ok(());
    }

    let config = PhantomTraceConfig::default();
    let mut processor = PhantomTraceProcessor::new(config)?;

    let result = processor.phantom_file(input_path, output_path)?;

    println!("✅ Processed {} lines", result.lines_processed);
    println!("👻 {} lines had data phantomed", result.lines_phantomed);
    println!("📁 Output saved to '{}'", output_path);

    Ok(())
}
