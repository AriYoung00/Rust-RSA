use std::io;
use num::BigUint;

use crate::rsa;

fn print_help() {
    println!("Key generation: ");
    println!("\t k -> generate new key and store in memory");
    println!("\t wk -> write key stored in memory to file");
    println!("\t rk <filename> -> read key from <filename>");

    println!("Encryption: ");
    println!("\t e <message> [filename] -> Encrypt message using stored key, optionally write resulting \
    \n\t\t\tcipher to [filename]. If no filename is provided, cipher will be written to stdout and stored in memory");

    println!("Decryption: ");
    println!("\t d -> Decrypt cipher stored in memory, display result to stdout");
    println!("\t df <filename> -> Read cipher from file and decrypt, display result to stdout");

    println!("Misc: ");
    println!("\t q -> Quit.");
    println!("\t s -> Print status. Shows whether key/cipher is stored in memory");
    println!("\t h -> Print this help menu again");
}


pub fn init_cli_interface() {
    println!("Rust implementation of RSA-1024, written by Ariel Young and Nashir Janmohamed\n");
    println!("Commands are as follows:");
    print_help();
    print!("\n> ");

    let mut stored_key: Option<((BigUint, BigUint), BigUint)> = None;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read input");
    let mut parts: Vec<&str> = input.split_ascii_whitespace().collect();
    loop {
        match parts[0] {
            "k" => {
                println!("Generating key...");
                stored_key = Some(rsa::gen_key());
                println!("Finished!");
            },
            "wk" => unimplemented!("Hello yes this is unimplemented"),
            "rk" => unimplemented!("Hello yes this is unimplemented as well"),

            "e" => {
                if parts.len() == 2 {
                    println!("Encryption message...");
                    let res = rsa::
                }
             },

            _ => println!("Input not recognized, enter 'h' for help"),
        }
    }
}