use crate::config::PhantomTraceConfig;
use crate::processor::PhantomTraceProcessor;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream}; // Added TcpStream import
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct StreamProcessor {
    processor: PhantomTraceProcessor,
    buffer_size: usize,
    flush_interval: Duration,
}

impl StreamProcessor {
    pub fn new(config: PhantomTraceConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let processor = PhantomTraceProcessor::new(config.clone())?;
        Ok(Self {
            processor,
            buffer_size: config.processing.batch_size,
            flush_interval: Duration::from_millis(100),
        })
    }

    pub fn process_stream(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let mut stdout_lock = stdout.lock();
        let mut buffer = Vec::new();

        for line in stdin.lock().lines() {
            let line = line?;
            buffer.push(line);

            if buffer.len() >= self.buffer_size {
                // Use buffer_size here
                for buffered_line in buffer.drain(..) {
                    let result = self.processor.phantom_text(&buffered_line);
                    writeln!(stdout_lock, "{}", result.phantomed_text)?;
                }
                stdout_lock.flush()?;
            }
        }

        // Process remaining items in buffer
        for buffered_line in buffer {
            let result = self.processor.phantom_text(&buffered_line);
            writeln!(stdout_lock, "{}", result.phantomed_text)?;
        }
        stdout_lock.flush()?;
        Ok(())
    }

    // File monitoring for log file preprocessing (fixed borrowing issue)
    pub fn process_file_stream(
        &mut self,
        input_path: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::{Seek, SeekFrom};

        let mut output = File::create(output_path)?;
        let mut last_pos = 0u64;

        loop {
            let mut file = File::open(input_path)?; // Reopen file each iteration
            file.seek(SeekFrom::Start(last_pos))?;

            let reader = BufReader::new(file);
            let mut new_pos = last_pos;

            for line in reader.lines() {
                let line = line?;
                let result = self.processor.phantom_text(&line);
                writeln!(output, "{}", result.phantomed_text)?;
                new_pos += line.len() as u64 + 1; // +1 for newline
            }

            last_pos = new_pos;
            output.flush()?;
            thread::sleep(self.flush_interval);
        }
    }

    // TCP server mode for network log ingestion
    pub fn serve_tcp(&mut self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
        println!("👻 PhantomTrace TCP server listening on port {}", port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Create a new processor for each connection
                    let config = self.processor.config.clone(); // Need to expose config in processor
                    thread::spawn(move || match PhantomTraceProcessor::new(config) {
                        Ok(mut processor) => {
                            if let Err(e) = handle_tcp_client(stream, &mut processor) {
                                eprintln!("Error handling client: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Error creating processor: {}", e),
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
        Ok(())
    }
}

// Fixed function signature and variable handling
fn handle_tcp_client(
    stream: TcpStream, // Removed mut since we'll clone it
    processor: &mut PhantomTraceProcessor,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut write_stream = stream.try_clone()?; // Clone for writing
    let reader = BufReader::new(stream); // Use original for reading

    for line in reader.lines() {
        let line = line?; // This now correctly gets String, not &str
        let result = processor.phantom_text(&line);
        writeln!(write_stream, "{}", result.phantomed_text)?;
    }
    Ok(())
}
