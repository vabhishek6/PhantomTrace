use regex::Regex;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug)]
pub struct PhantomTracer {
    compiled_rules: Vec<CompiledTraceRule>,
    trace_stats: HashMap<String, TraceStats>,
    phantom_tokens: HashMap<String, String>, // For consistent tokenization
}

#[derive(Debug)]
struct CompiledTraceRule {
    name: String,
    regex: Regex,
    method: ObfuscationMethod,
    preserve_chars: Option<usize>,
    replacement: Option<String>,
    severity: TraceSeverity,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct TraceStats {
    pub phantoms_created: u64,
    pub characters_traced: u64,
    pub severity_level: String,
    pub first_trace: Option<std::time::SystemTime>,
    pub last_trace: Option<std::time::SystemTime>,
}

impl PhantomTracer {
    pub fn new(rules: &[TraceRule]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut compiled_rules = Vec::new();
        let mut trace_stats = HashMap::new();

        for rule in rules {
            let regex = if rule.case_sensitive.unwrap_or(false) {
                Regex::new(&rule.pattern)?
            } else {
                Regex::new(&format!("(?i){}", rule.pattern))?
            };
            
            compiled_rules.push(CompiledTraceRule {
                name: rule.name.clone(),
                regex,
                method: rule.method.clone(),
                preserve_chars: rule.preserve_chars,
                replacement: rule.replacement.clone(),
                severity: rule.severity.clone(),
            });
            
            trace_stats.insert(rule.name.clone(), TraceStats {
                severity_level: format!("{:?}", rule.severity),
                ..Default::default()
            });
        }

        Ok(Self {
            compiled_rules,
            trace_stats,
            phantom_tokens: HashMap::new(),
        })
    }

    pub fn trace_and_phantom(&mut self, text: &str) -> (String, Vec<PhantomEvent>) {
        let mut result = text.to_string();
        let mut events = Vec::new();

        // Process rules by severity (Critical first)
        let mut sorted_rules = self.compiled_rules.clone();
        sorted_rules.sort_by(|a, b| {
            let a_priority = match a.severity {
                TraceSeverity::Critical => 0,
                TraceSeverity::High => 1,
                TraceSeverity::Medium => 2,
                TraceSeverity::Low => 3,
            };
            let b_priority = match b.severity {
                TraceSeverity::Critical => 0,
                TraceSeverity::High => 1,
                TraceSeverity::Medium => 2,
                TraceSeverity::Low => 3,
            };
            a_priority.cmp(&b_priority)
        });

        for rule in &sorted_rules {
            let original_result = result.clone();
            
            result = rule.regex.replace_all(&result, |caps: &regex::Captures| {
                let matched = caps.get(0).map_or("", |m| m.as_str());
                let phantomed = self.apply_obfuscation(matched, &rule.method, 
                                                     rule.preserve_chars, &rule.replacement);
                
                // Record the phantom event
                events.push(PhantomEvent {
                    rule_name: rule.name.clone(),
                    severity: rule.severity.clone(),
                    original_value: matched.to_string(),
                    phantom_value: phantomed.clone(),
                    position: caps.get(0).map(|m| (m.start(), m.end())).unwrap_or((0, 0)),
                    trace_id: generate_trace_id(),
                });

                phantomed
            }).to_string();

            // Update statistics if changes were made
            if result != original_result {
                let stats = self.trace_stats.get_mut(&rule.name).unwrap();
                stats.phantoms_created += 1;
                stats.characters_traced += original_result.len() as u64 - result.len() as u64;
                
                let now = std::time::SystemTime::now();
                if stats.first_trace.is_none() {
                    stats.first_trace = Some(now);
                }
                stats.last_trace = Some(now);
            }
        }

        (result, events)
    }

    fn apply_obfuscation(&mut self, value: &str, method: &ObfuscationMethod, 
                        preserve_chars: Option<usize>, replacement: &Option<String>) -> String {
        match method {
            ObfuscationMethod::Phantom => {
                let preserve = preserve_chars.unwrap_or(0);
                phantom_string(value, preserve)
            },
            ObfuscationMethod::Mirror => {
                format!("PHANTOM_{:08X}", phantom_hash(value))
            },
            ObfuscationMethod::Mask => {
                replacement.clone().unwrap_or_else(|| "[PHANTOMED]".to_string())
            },
            ObfuscationMethod::Vanish => {
                String::new()
            },
            ObfuscationMethod::Tokenize => {
                // Consistent tokenization
                let token_key = format!("token_{}", phantom_hash(value));
                if let Some(existing_token) = self.phantom_tokens.get(&token_key) {
                    existing_token.clone()
                } else {
                    let token = format!("PHANTOM_TOKEN_{:08X}", phantom_hash(value));
                    self.phantom_tokens.insert(token_key, token.clone());
                    token
                }
            },
        }
    }

    pub fn get_trace_report(&self) -> TraceReport {
        let mut total_phantoms = 0;
        let mut total_characters_traced = 0;
        let mut severity_breakdown = HashMap::new();

        for (rule_name, stats) in &self.trace_stats {
            total_phantoms += stats.phantoms_created;
            total_characters_traced += stats.characters_traced;
            
            *severity_breakdown.entry(stats.severity_level.clone()).or_insert(0u64) += stats.phantoms_created;
        }

        TraceReport {
            total_phantoms_created: total_phantoms,
            total_characters_traced: total_characters_traced,
            rules_triggered: self.trace_stats.iter()
                .filter(|(_, stats)| stats.phantoms_created > 0)
                .count(),
            severity_breakdown,
            detailed_stats: self.trace_stats.clone(),
            generation_time: std::time::SystemTime::now(),
        }
    }

    pub fn reset_traces(&mut self) {
        for stats in self.trace_stats.values_mut() {
            *stats = TraceStats {
                severity_level: stats.severity_level.clone(),
                ..Default::default()
            };
        }
        self.phantom_tokens.clear();
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PhantomEvent {
    pub rule_name: String,
    pub severity: TraceSeverity,
    pub original_value: String,
    pub phantom_value: String,
    pub position: (usize, usize),
    pub trace_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TraceReport {
    pub total_phantoms_created: u64,
    pub total_characters_traced: u64,
    pub rules_triggered: usize,
    pub severity_breakdown: HashMap<String, u64>,
    pub detailed_stats: HashMap<String, TraceStats>,
    pub generation_time: std::time::SystemTime,
}

// Utility functions for phantoming
fn phantom_string(input: &str, preserve: usize) -> String {
    let len = input.len();
    if len <= preserve * 2 {
        "█".repeat(len) // Use block character for "phantom" effect
    } else {
        format!(
            "{}{}{}",
            &input[..preserve],
            "█".repeat(len - preserve * 2),
            &input[len - preserve..]
        )
    }
}

fn phantom_hash(input: &str) -> u32 {
    // Simple but effective hash function (not cryptographic)
    let mut hash = 2166136261u32;
    for byte in input.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(16777619);
    }
    hash
}

fn generate_trace_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("TRACE_{:016X}", timestamp & 0xFFFFFFFFFFFFFFFF)
}

// Re-export types from config
use crate::config::{TraceRule, ObfuscationMethod, TraceSeverity};
