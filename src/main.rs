use rand::{seq::SliceRandom, thread_rng};
use std::fs::{self, File};
use std::io::{self, BufRead, Write, BufWriter};

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

        let mut first = 0;
        for line in reader.lines() {
            let line = line?;
            let elements: Vec<_> = line.split(' ').collect();

            match elements[0] {
                "c" => {}
                _ => {
                    if first == 0 {
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
                        first += 1;
                    } else if  first == 1 {
                        match elements[0].parse::<usize>() {
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
                        first += 1;
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
    /*if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        return Ok(());
    }*/

    // Parse command line arguments
    //let input_file = &args[1];
    //let output_file = &args[2];
    //let k: usize = args[3].parse().expect("k must be a valid usize");
    let mut counter = 0;

    for file in fs::read_dir("~/Testgraphen/random_planar_graphs/10M/").expect("no such file or directory") {
        let file_name = file.unwrap().file_name();

        let file = File::open(format!("~/Testgraphen/random_planar_graphs/10M/{}", file_name.to_str().unwrap()))?;
        let mut reader = io::BufReader::new(file);

        // Open output file for writing
        let output = File::create(format!("./augmented_testgraphs/{}/{}", 10 + counter / 10, file_name.to_str().unwrap()))?;
        let mut writer = BufWriter::new(output);

        let gf = GraphFile::read(&mut reader)?;
        let n = gf.n;
        let sqrt = (n as f64).sqrt() as usize;

        if counter < 10 {
            gf.augment(n).write(&mut writer)?;
        } else if counter < 20 {
            gf.augment(sqrt).write(&mut writer)?;
        } else  {
            break;
        }

        counter += 1
    }

    // Open input file for reading
    

    Ok(())
}
