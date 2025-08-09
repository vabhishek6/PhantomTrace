use phantomtrace::{PhantomTraceConfig, PhantomTraceProcessor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"
    Name: Alice Smith
    Credit Card: 4111-1111-1111-1111
    Email: alice@example.com
    Phone: 555-123-4567
    "#;

    let mut processor = PhantomTraceProcessor::new(PhantomTraceConfig::default())?;
    let _ = processor.phantom_text(input);

    let report = processor.get_trace_report();

    println!("=== PhantomTrace Report ===");
    println!("Total Phantoms: {}", report.total_phantoms_created);
    println!("Rules Triggered: {}", report.rules_triggered);
    println!("Severity Breakdown: {:?}", report.severity_breakdown);
    println!("===========================");

    Ok(())
}
