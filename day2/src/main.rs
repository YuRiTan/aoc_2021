use anyhow::Result;
use std::fs::File;
use std::io::Read;

fn q1() -> Result<()> {
    let mut f = File::open("../data/day2_input.txt")?;
    let mut data = String::new();
    f.read_to_string(&mut data).expect("could not read file");

    let numbers = data.lines().map(|line| {
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let steps = split.next().unwrap().parse::<isize>().unwrap();
        (direction, steps)
    });

    let forward_sum = numbers
        .clone()
        .filter(|pair| pair.0 == "forward")
        .map(|pair| pair.1)
        .sum::<isize>();

    let up_sum = numbers
        .clone()
        .filter(|pair| pair.0 == "up")
        .map(|pair| pair.1)
        .sum::<isize>();

    let down_sum = numbers
        .filter(|pair| pair.0 == "down")
        .map(|pair| pair.1)
        .sum::<isize>();

    let answer = forward_sum * (down_sum - up_sum);
    println!("The product of the final position is: {}", answer);

    Ok(())
}

fn q2() -> Result<()> {
    let mut f = File::open("../data/day2_input.txt")?;
    let mut data = String::new();
    f.read_to_string(&mut data).expect("could not read file");

    let numbers = data.lines().map(|line| {
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let steps = split.next().unwrap().parse::<isize>().unwrap();
        (direction, steps)
    });

    let forward = numbers
        .clone()
        .filter(|pair| pair.0 == "forward")
        .map(|pair| pair.1);

    let up = numbers
        .clone()
        .filter(|pair| pair.0 == "up")
        .map(|pair| pair.1);

    let down = numbers
        .clone()
        .filter(|pair| pair.0 == "down")
        .map(|pair| pair.1);

    let height = down.clone().zip(up.clone()).map(|(d, u)| d - u);
    let aim = forward.clone().zip(height).map(|(f, h)| f * h);

    let answer =
        forward.sum::<isize>() * (down.sum::<isize>() - up.sum::<isize>() + aim.sum::<isize>());
    println!(
        "The product of the final position (with aim) is: {}",
        answer
    );

    Ok(())
}

fn main() -> Result<()> {
    q1()?;
    q2()?;

    Ok(())
}
