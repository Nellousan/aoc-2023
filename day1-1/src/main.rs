use std::{
    error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        Err("Missing an argument")?;
    }
    let file = File::open(args[1].clone())?;
    let lines = io::BufReader::new(file).lines();

    let mut res = 0;
    for line in lines {
        if let Ok(line) = line {
            let stripped: String = line.chars().filter(|x| x.is_numeric()).collect();
            let (_, mut stripped): (Vec<usize>, String) = stripped
                .chars()
                .enumerate()
                .filter(|&(i, _)| i == 0 || i == stripped.len() - 1)
                .unzip();

            if stripped.len() == 1 {
                stripped.push(stripped.chars().nth(0).unwrap());
            }

            res += stripped.parse::<u32>().unwrap();
        }
    }

    println!("{}", res);

    Ok(())
}
