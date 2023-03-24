use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};

fn main() -> Result<(), Box<dyn Error>> {
    // Open input file
    let input_file = File::open("input.csv")?;
    let input_file = BufReader::new(input_file);

    // Open output file
    let output_file = File::create("output.csv")?;
    let mut output_file = BufWriter::new(output_file);

    // Process input file
    for line in input_file.lines() {
        let line = line?;

        // Split line into fields
        let fields: Vec<&str> = line.split(',').collect();

        // Parse values from fields
        let x: f64 = fields[0].parse()?;
        let y: f64 = fields[1].parse()?;
        let z: f64 = fields[2].parse()?;

        // Perform calculations
        let sum = x + y + z;
        let product = x * y * z;

        // Write results to output file
        writeln!(output_file, "{:.2},{:.2}", sum, product)?;
    }

    Ok(())
}
