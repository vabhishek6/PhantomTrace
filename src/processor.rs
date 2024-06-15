use crate::config::{PhantomTraceConfig, OutputFormat};
use crate::tracer::{PhantomTracer, PhantomEvent, TraceReport};
use std::time::Instant;
use serde::Serialize;

#[derive(Debug)]
pub struct PhantomTraceProcessor {
    config: PhantomTraceConfig,
    tracer: PhantomTracer,
    processing_stats: ProcessingStats,
}

#[derive(Debug, Default)]
pub struct ProcessingStats {
    pub lines_processed: u64,
    pub lines_phantomed: u64,
    pub total_phantom_events: u64,
    pub processing_time: std::time::Duration,
    pub start_time: Option<Instant>,
}

impl PhantomTraceProcessor {
    pub fn new(config: PhantomTraceConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let tracer = PhantomTracer::new(&config.tracing.rules)?;
        
        Ok(Self {
            config,
            tracer,
            processing_stats: ProcessingStats::default(),
        })
    }

    pub fn phantom_text(&mut self, input: &str) -> ProcessingResult {
        let start_time = Instant::now();
        
        if self.processing_stats.start_time.is_none() {
            self.processing_stats.start_time = Some(start_time);
        }

        let lines: Vec<&str> = input.lines().collect();
        let mut phantomed_lines = Vec::new();
        let mut all_events = Vec::new();
        let mut lines_phantomed = 0;

        for line in lines {
            let (phantomed_line, events) = self.tracer.trace_and_phantom(line);
            
            if !events.is_empty() {
                lines_phantomed += 1;
                all_events.extend(events);
            }
            
            phantomed_lines.push(phantomed_line);
        }

        let processing_time = start_time.elapsed();
        
        // Update stats
        self.processing_stats.lines_processed += phantomed_lines.len() as u64;
        self.processing_stats.lines_phantomed += lines_phantomed;
        self.processing_stats.total_phantom_events += all_events.len() as u64;
        self.processing_stats.processing_time += processing_time;

        ProcessingResult {
            phantomed_text: phantomed_lines.join("\n"),
            phantom_events: all_events,
            lines_processed: phantomed_lines.len(),
            lines_phantomed: lines_phantomed as usize,
            processing_time,
        }
    }

    pub fn phantom_file(&mut self, input_path: &str, output_path: &str) -> Result<ProcessingResult, Box<dyn std::error::Error>> {
        let input_content = std::fs::read_to_string(input_path)?;
        let result = self.phantom_text(&input_content);
        
        // Write output based on format
        match self.config.output.format {
            OutputFormat::Text => {
                std::fs::write(output_path, &result.phantomed_text)?;
            },
            OutputFormat::Json => {
                let json_output = serde_json::to_string_pretty(&JsonOutput {
                    phantomed_text: result.phantomed_text.clone(),
                    events: if self.config.output.log_phantom_events { 
                        Some(result.phantom_events.clone()) 
                    } else { 
                        None 
                    },
                    trace_report: if self.config.output.include_trace_report { 
                        Some(self.get_trace_report()) 
                    } else { 
                        None 
                    },
                })?;
                std::fs::write(output_path, json_output)?;
            },
            OutputFormat::Csv => {
                let mut csv_content = String::new();
                csv_content.push_str("rule_name,severity,original_value,phantom_value,start_pos,end_pos,trace_id\n");
                for event in &result.phantom_events {
                    csv_content.push_str(&format!(
                        "{},{:?},{},{},{},{},{}\n",
                        event.rule_name,
                        event.severity,
                        event.original_value,
                        event.phantom_value,
                        event.position.0,
                        event.position.1,
                        event.trace_id
                    ));
                }
                std::fs::write(output_path, csv_content)?;
            },
            OutputFormat::TraceReport => {
                let report = self.get_trace_report();
                let report_json = serde_json::to_string_pretty(&report)?;
                std::fs::write(output_path, report_json)?;
            },
        }

        // Create trace map if requested
        if self.config.output.create_trace_map {
            let trace_map_path = format!("{}.tracemap", output_path);
            self.create_trace_map(&result, &trace_map_path)?;
        }

        Ok(result)
    }

    fn create_trace_map(&self, result: &ProcessingResult, map_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let trace_map = TraceMap {
            total_events: result.phantom_events.len(),
            events_by_severity: {
                let mut map = std::collections::HashMap::new();
                for event in &result.phantom_events {
                    *map.entry(format!("{:?}", event.severity)).or_insert(0) += 1;
                }
                map
            },
            events_by_rule: {
                let mut map = std::collections::HashMap::new();
                for event in &result.phantom_events {
                    *map.entry(event.rule_name.clone()).or_insert(0) += 1;
                }
                map
            },
            phantom_coverage: if result.lines_processed > 0 {
                (result.lines_phantomed as f64 / result.lines_processed as f64) * 100.0
            } else {
                0.0
            },
        };

        let trace_map_json = serde_json::to_string_pretty(&trace_map)?;
        std::fs::write(map_path, trace_map_json)?;
        Ok(())
    }

    pub fn get_trace_report(&self) -> TraceReport {
        self.tracer.get_trace_report()
    }

    pub fn get_processing_stats(&self) -> ProcessingStatsOutput {
        ProcessingStatsOutput {
            lines_processed: self.processing_stats.lines_processed,
            lines_phantomed: self.processing_stats.lines_phantomed,
            total_phantom_events: self.processing_stats.total_phantom_events,
            processing_time_ms: self.processing_stats.processing_time.as_millis() as u64,
            trace_report: self.get_trace_report(),
        }
    }

    pub fn reset_stats(&mut self) {
        self.processing_stats = ProcessingStats::default();
        self.tracer.reset_traces();
    }
}

#[derive(Debug)]
pub struct ProcessingResult {
    pub phantomed_text: String,
    pub phantom_events: Vec<PhantomEvent>,
    pub lines_processed: usize,
    pub lines_phantomed: usize,
    pub processing_time: std::time::Duration,
}

#[derive(Debug, Serialize)]
struct JsonOutput {
    phantomed_text: String,
    events: Option<Vec<PhantomEvent>>,
    trace_report: Option<TraceReport>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProcessingStatsOutput {
    pub lines_processed: u64,
    pub lines_phantomed: u64,
    pub total_phantom_events: u64,
    pub processing_time_ms: u64,
    pub trace_report: TraceReport,
}

#[derive(Debug, Serialize)]
struct TraceMap {
    total_events: usize,
    events_by_severity: std::collections::HashMap<String, usize>,
    events_by_rule: std::collections::HashMap<String, usize>,
    phantom_coverage: f64,
}
