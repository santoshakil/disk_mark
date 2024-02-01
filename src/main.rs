use std::fs::File;
use std::io::Write;
use std::time::Instant;

const CHUNK_SIZE: usize = 1024 * 1024 * 10; // 10 MB

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Please provide a file path as an 1st argument and a size in GB as an 2nd");
        return;
    }

    match args[2].parse() {
        Ok(size) => {
            if let Err(err) = benchmark_read_write(&args[1], size) {
                eprintln!("Error: {}", err);
            }
        }
        Err(_) => eprintln!("Error: Size argument is not a valid number"),
    }
}

fn benchmark_read_write(file_path: &str, size: u8) -> Result<(), std::io::Error> {
    let total_size = (size as usize) * 1024 * 1024 * 1024;
    let buffer = vec![0; CHUNK_SIZE];
    let mut file = File::create(file_path)?;
    let mut written_bytes = 0;

    let start = Instant::now();
    for _ in 0..(total_size / CHUNK_SIZE) {
        file.write_all(&buffer)?;
        written_bytes += CHUNK_SIZE;
        log_speed(written_bytes, start.elapsed());
    }
    let duration = start.elapsed();
    let speed = written_bytes as f64 / duration.as_secs_f64() / 1024.0 / 1024.0;

    let start = Instant::now();
    std::fs::remove_file(file_path)?;
    let duration = start.elapsed();
    println!("\nAverage speed: {:.2} MB/s", speed);
    println!("Deleted {} GB in {:?} ", size, duration);
    println!("Time took: {:?}", duration);

    Ok(())
}

// Corrected function name
fn log_speed(bytes: usize, duration: std::time::Duration) {
    let speed = bytes as f64 / duration.as_secs_f64();
    println!("Speed: {:.2} MB/s", speed / 1024.0 / 1024.0);
}
