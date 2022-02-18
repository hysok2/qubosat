use std::fs::File;
use std::io::{BufRead,BufReader};

pub fn readqubo(filename: String) -> Result<Vec<Vec<i32>>, String> {
    let file = File::open(filename).map_err(|_| "file open error")?;
    let reader = BufReader::new(&file);

    let mut clauses: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|_| "read line error")?;
        let vec: Vec<&str> = line.split_whitespace().collect();
        //println!("{:?}", vec);
        if vec.len() == 0 {
            continue;
        }
        if vec[0] != "c" && vec.len() > 1 {
            clauses.push(vec.iter().map(|c| c.parse::<i32>()).collect::<Result<Vec<i32>,_>>().map_err(|_| "parse error")?);
        }

    }
    Ok(clauses)
}
