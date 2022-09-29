use std::io::prelude::BufRead;

fn print_progress_bar(count: u32, total: u32, what: &str) {
    println!("{} -> {} for '{}'", count, total, what);
}

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();

    let re = regex::Regex::new(r"^\[(\d+)/(\d+)\] (.*)").unwrap();

    for line in input.lock().lines() {
        let l = line.unwrap();
        match re.captures(&l) {
            None => {
                println!("{}", l);
            }
            Some(x) => {
                let count: u32 = x[1].parse::<u32>().unwrap();
                let total: u32 = x[2].parse::<u32>().unwrap();
                let what = &x[3];
                print_progress_bar(count, total, what);
            }
        }
    }

    Ok(())
}
