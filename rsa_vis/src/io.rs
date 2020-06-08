use std::{fs, fs::File};
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use num::{BigUint};
use crate::rsa;

#[derive(Serialize, Deserialize)]
struct PublicKey {
    n: BigUint,
    e: BigUint,
}

#[derive(Serialize, Deserialize)]
struct PrivateKey {
    d: BigUint
}

#[derive(Serialize, Deserialize)]
struct Cipher {
    msg: Vec<BigUint>
}

pub fn write_json_to_disk(key: ((BigUint, BigUint), BigUint)) {
    let pub_key = json!({
        "n": (key.0).0,
        "e": (key.0).1
    });
    // println!("key n: {:?}", (key.0).0);
    // println!("json: {}", pub_key.to_string());
    _write_json_to_disk(&pub_key, "pub_key.txt").expect("Something went wrong writing the file");

    let priv_key = json!({
        "d": key.1
    });
    // println!("key d: {:?}", key.1);
    // println!("json: {}", priv_key.to_string());
    _write_json_to_disk(&priv_key, "priv_key.txt").expect("Something went wrong writing the file");
}

fn _write_json_to_disk(key: &Value, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(key.to_string().as_ref())?;
    Ok(())
}

pub fn read_key_from_disk() -> std::io::Result<((BigUint, BigUint), BigUint)> {
    let pub_key_str = fs::read_to_string("pub_key.txt")?;
    let pub_key: PublicKey = serde_json::from_str(&pub_key_str)?;
    // println!("json: {:?}", pub_key.n);
    let priv_key_str = fs::read_to_string("priv_key.txt")?;
    let priv_key: PrivateKey = serde_json::from_str(&priv_key_str)?;
    // println!("json: {:?}", priv_key.d);

    Ok(((pub_key.n, pub_key.e), priv_key.d))
}

pub fn write_cipher_to_disk(cipher: &Vec<BigUint>, dest_path: &str) {
    let cipher_json = json!({
        "msg": cipher
    });

    _write_json_to_disk(&cipher_json, dest_path)
        .expect("Something went wrong reading the file");
}

pub fn read_cipher_from_disk(src_path: &str) -> std::io::Result<Vec<BigUint>>{
    let cipher_str = fs::read_to_string(src_path)
        .expect("Something went wrong reading the file");
    let cipher: Cipher = serde_json::from_str(&cipher_str)?;

    Ok(cipher.msg)
}

pub fn encrypt_file(src_path: &str, dest_path: &str, pub_key: (BigUint, BigUint)) {
    let msg = fs::read_to_string(src_path)
        .expect("Something went wrong reading the file");
    let encrypted_msg = rsa::encrypt_str(&msg, pub_key);

    let cipher = json!({
        "msg": encrypted_msg
    });

    _write_json_to_disk(&cipher, dest_path).expect("Something went wrong reading the file");
}

pub fn decrypt_file(src_path: &str, dest_path: &str, priv_key: BigUint, pub_key: BigUint) -> std::io::Result<()> {
    let cipher_str = fs::read_to_string(src_path)
        .expect("Something went wrong reading the file");
    let cipher: Cipher = serde_json::from_str(&cipher_str).unwrap();

    // TODO: make this work :o
    println!("cipher message");
    for thing in cipher.msg.clone() {
        println!("{}", thing);
    }

    let decrypted_msg = rsa::decrypt_str(&cipher.msg, priv_key, pub_key);
    let mut dest = File::create(dest_path)?;
    dest.write_all(decrypted_msg.as_ref())?;
    Ok(())
}
