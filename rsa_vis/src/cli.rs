use std::io;
use num::BigUint;

use crate::{rsa, io as asdf};
use std::error::Error;

fn print_help() {
    println!("Key generation: ");
    println!("\t k -> generate new key and store in memory");
    println!("\t wk -> write key stored in memory to file");
    println!("\t rk <filename> -> read key from <filename>");

    println!("Encryption: ");
    println!("\t e <message> -> Encrypt message using stored key, storing cipher in memory.");
    println!("\t wc <filename> -> Write stored cipher to <filename>");
    println!("\t pc -> Prints stored cipher to stdout. Warning: very long line");

    println!("Decryption: ");
    println!("\t d -> Decrypt cipher stored in memory, display result to stdout");
    println!("\t df <filename> -> Read cipher from file and decrypt using stored key, display result to stdout");

    println!("Misc: ");
    println!("\t q -> Quit.");
    println!("\t s -> Print status. Shows whether key/cipher is stored in memory");
    println!("\t h -> Print this help menu again");
}


pub fn init_cli_interface() {
    println!("Rust implementation of RSA-1024, written by Ariel Young and Nashir Janmohamed\n");
    println!("Commands are as follows -- ");
    print_help();

    let mut stored_key: Option<((BigUint, BigUint), BigUint)> = None;
    let mut stored_cipher: Option<Vec<BigUint>> = None;


    'outer: loop {
        println!();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("> Unable to read input");
        let mut parts: Vec<&str> = input.split_ascii_whitespace().collect();

        match parts[0] {
            "k" => {
                println!("> Generating key...");
                stored_key = Some(rsa::gen_key());
                println!("> Finished!");
            },
            "wk" => match stored_key.clone() {
                Some(key) => {asdf::write_key_to_disk(key); println!("> Done!");},
                None => println!("> Error: No stored key")
            },
            "rk" => match asdf::read_key_from_disk() {
                Ok(key) => {stored_key = Some(key); println!("> Done!");},
                Err(e) => println!("> Error reading key from file: {}", e.to_string()),
            },

            "e" => {
                if stored_key.is_none() {
                    println!("> Error: No stored key");
                    println!("> Either generate one, or read from file using 'rk <filename>'");
                    continue 'outer;
                }
                if parts.len() >= 2 {
                    println!("> Encrypting message...");
                    let key = stored_key.clone().unwrap();
                    let res = rsa::encrypt_str(&parts[1..].join(" "), key.0);
                    println!("> Finished!");
                    println!("> Result: {:?}", res);
                    stored_cipher = Some(res);
                } else {
                    println!("> Error: invalid parameters to 'e'");
                }
             },

            "wc" => unimplemented!("> Writing cipher to file not yet implemented"),

            "pc" => match stored_cipher.clone() {
                Some(t) => {
                    println!("> Stored cipher: {:?}", t);
                    print!("> Hex: ");
                    for b in &t { print!("{:x}", b); }
                    println!();
                },
                None => println!("> Error: No stored cipher"),
            }

            "d" => {
                if stored_cipher.is_none() {
                    println!("> Error: No stored cipher.");
                    println!("> Either encrypt a message using 'e', or decrypt a cipher from file using 'df'");
                } else if stored_key.is_none() {
                    println!("> Error: No stored key");
                    println!("> You probably want to read one from disk using 'rk'");
                } else {
                    let key = stored_key.clone().unwrap();
                    println!("> Decryption result: {}", rsa::decrypt_str(&stored_cipher.clone().unwrap(),
                                                                       key.1, (key.0).0));
                }


            },

            "df" => unimplemented!("> Decrypting cipher from file not yet implemented"),

            "s" => {
                if stored_key.is_none() {
                    println!("> Key stored in memory: no");
                } else {
                    println!("> Key stored in memory: yes");
                }
                if stored_cipher.is_none() {
                    println!("> Key stored in memory: no");
                } else {
                    println!("> Key stored in memory: yes");
                }
            },

            "h" => print_help(),

            "q" => {
                println!("> Exiting...");
                return;
            },

            _ => println!("> Input not recognized, enter 'h' for help"),
        }
    }
}