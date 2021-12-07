use anyhow::Result;
use std::fs::File;
use std::io::Read;


fn q1() -> Result<()> {
    let mut f = File::open("../data/day1_input.txt")?;
    let mut data = String::new();
    f.read_to_string(&mut data).expect("could not read file");

    let numbers: Vec<usize> = data.lines().map(|line| line.parse().unwrap()).collect();

    let increases: Vec<bool> = numbers
        .windows(2)
        .map(|pair| pair[0] < pair[1] )
        .collect();

    let correct_count: usize = increases.iter().filter(|&n| *n).count();
    println!("Number of increases is: {}", correct_count);

    Ok(())

}


fn q2() -> Result<()> {
    let mut f = File::open("../data/day1_input.txt")?;
    let mut data = String::new();
    f.read_to_string(&mut data).expect("could not read file");

    let numbers: Vec<usize> = data.lines().map(|line| line.parse().unwrap()).collect();

    let sums3: Vec<usize> = numbers
        .windows(3)
        .map(|pair| pair.iter().sum() )
        .collect();

    let increases: Vec<bool> = sums3
        .windows(2)
        .map(|pair| pair[0] < pair[1] )
        .collect();

    let correct_count: usize = increases.iter().filter(|&n| *n).count();
    println!("Number of increases is: {}", correct_count);

    Ok(())
}


fn main() -> Result<()> {
    q1()?;
    q2()?;
    Ok(())
}
