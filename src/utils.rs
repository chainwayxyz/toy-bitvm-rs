use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::str::FromStr;

use bitcoin::secp256k1::{All, Secp256k1};
use bitcoin::taproot::{TaprootBuilder, TaprootSpendInfo};
use bitcoin::{Address, ScriptBuf, XOnlyPublicKey};



pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn take_stdin(prompt: &str) -> String {
    print!("{}", prompt);
    let mut string = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read txid");
    string.trim().to_string()
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

pub fn taproot_address_from_script_leaves(
    secp: &Secp256k1<All>,
    scripts: Vec<ScriptBuf>,
) -> (Address, TaprootSpendInfo) {
    let n = scripts.len();
    assert!(n > 1, "more than one script is required");
    let m: u8 = ((n - 1).ilog2() + 1) as u8; // m = ceil(log(n))
    let k = 2_usize.pow(m.into()) - n;
    let taproot = (0..n).fold(TaprootBuilder::new(), |acc, i| {
        acc.add_leaf(m - ((i >= n - k) as u8), scripts[i].clone())
            .unwrap()
    });
    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )
    .unwrap();
    let tree_info = taproot.finalize(secp, internal_key).unwrap();
    let address = Address::p2tr(
        secp,
        internal_key,
        tree_info.merkle_root(),
        bitcoin::Network::Signet,
    );
    (address, tree_info)
}
