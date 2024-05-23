use rand::{seq::SliceRandom, thread_rng};
use std::fs::File;
use std::io::{self, BufRead, Read, Write, BufWriter};

struct GraphFile {
    lines: Vec<String>,
    n: usize,
    m: usize,
}

impl GraphFile {
    fn read<T: BufRead>(reader: &mut T) -> Result<GraphFile, std::io::Error> {
        let mut n: usize = 0;
        let mut m: usize = 0;
        let mut lines: Vec<String> = vec![];

        let mut first = false;
        for line in reader.lines() {
            let line = line?;
            let elements: Vec<_> = line.split(' ').collect();

            match elements[0] {
                "c" => {}
                _ => {
                    if !first {
                        match elements[0].parse::<usize>() {
                            Ok(val) => {
                                n = val;
                            }
                            Err(_) => {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    "Invalid order of graph",
                                ))
                            }
                        }
                        match elements[1].parse::<usize>() {
                            Ok(val) => {
                                m = val;
                            }
                            Err(_) => {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    "Invalid num edges of graph",
                                ))
                            }
                        }
                        first = true;
                    } else {
                        lines.push(line);
                    }
                }
            }
        }
        Ok(GraphFile { lines, n, m })
    }

    fn augment(mut self, k: usize) -> Self {
        let mut nums: Vec<usize> = (1..(self.n + 1)).collect();
        nums.shuffle(&mut thread_rng());
        for i in 0..k {
            let v = self.n + 1 + i;
            self.lines.push(format!("{} {}", v, nums[i]));
            self.m += 1;
        }
        self
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        writeln!(writer, "{} {}", self.n, self.m)?;
        for line in &self.lines {
            writeln!(writer, "{}", line.as_str())?;
        }
        Ok(())
    }
}
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    // Check if enough arguments are provided
    if args.len() < 4 {
        eprintln!("Usage: {} <input_file> <output_file> <k>", args[0]);
        return Ok(());
    }

    // Parse command line arguments
    let input_file = &args[1];
    let output_file = &args[2];
    let k: usize = args[3].parse().expect("k must be a valid usize");

    // Open input file for reading
    let file = File::open(input_file)?;
    let mut reader = io::BufReader::new(file);

    // Open output file for writing
    let output = File::create(output_file)?;
    let mut writer = BufWriter::new(output);

    GraphFile::read(&mut reader)?.augment(k).write(&mut writer)?;

    Ok(())
}
