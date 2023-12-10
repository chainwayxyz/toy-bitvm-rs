use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
pub fn take_stdin<T: std::str::FromStr>(prompt: &str) -> Result<T, T::Err> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");

    string.trim().parse::<T>()
}

pub fn number_to_bool_array(number: usize, length: usize) -> Vec<bool> {
    let mut v = Vec::new();
    for i in 0..length {
        v.push(0 != number & (1 << i));
    }
    v
}

pub fn bool_array_to_number(bool_array: Vec<bool>) -> usize {
    let mut a = 0;
    for b in bool_array.iter().rev() {
        a *= 2;
        if *b {
            a += 1;
        }
    }
    a
}

pub fn hex_string_to_bool_array(hex: String) -> Vec<bool> {
    let a = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    let mut v = Vec::new();
    for c in hex.to_ascii_lowercase().chars() {
        let i = a.iter().position(|&x| x == c);
        assert!(i.is_some(), "non hex character");
        let mut z = number_to_bool_array(i.unwrap(), 4);
        z.reverse();
        v.extend(z);
    }
    v
}

pub fn bool_array_to_hex_string(bool_array: Vec<bool>) -> String {
    let a = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    assert!(bool_array.len() % 8 == 0, "array length is not compatible");
    let mut v = Vec::<char>::new();
    for i in 0..(bool_array.len() / 4) {
        let p = &mut bool_array[(4 * i)..(4 * i + 4)].to_vec();
        p.reverse();
        let u = bool_array_to_number(p.to_vec());
        v.push(a[u]);
    }
    v.into_iter().collect::<String>()
}
