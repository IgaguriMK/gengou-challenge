use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Read, Write};

use failure::Error;

type Result<T> = std::result::Result<T, Error>;

fn main() {
    match wrapped_main() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn wrapped_main() -> Result<()> {
    let chars = load_char_list()?;

    let f = File::create("gengoul.csv")?;
    let mut w = BufWriter::new(f);

    let n = chars.len();
    let mut c = 0usize;
    for i in 0..n {
        for j in 0..n {
            writeln!(&mut w, "{},{}{}", c, chars[i], chars[j])?;
            c += 1;
        }
    }

    Ok(())
}

fn load_char_list() -> Result<Vec<String>> {
    let f = File::open("joyo2010.csv")?;
    let mut r = BufReader::new(f);
    
    // read BOM
    let mut bs = [0u8; 3];
    r.read(&mut bs)?;

    let mut res = Vec::new();

    for l in r.lines() {
        res.push(l?);
    }

    Ok(res)
}