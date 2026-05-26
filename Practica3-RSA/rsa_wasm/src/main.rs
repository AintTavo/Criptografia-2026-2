// librerias estandar
use std::fs::File;
use std::io::Write;

use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};


fn main() {

    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate");
    let pub_key = RsaPublicKey::from(&priv_key);
    
    println!("Hello, world!");
}

fn gen_keys(length : usize) -> Result<(RsaPrivateKey, RsaPublicKey), Box<dyn std::error::Error>> {
    match length {
        1024 | 2048 | 3072 | 4096 => {
            let mut rng = rand::thread_rng();
            let priv_key = RsaPrivateKey::new(&mut rng, length).expect("failed to generate");
            let pub_key = RsaPublicKey::from(&priv_key);
            
            
            return Ok((priv_key, pub_key))
        },
        _ => return Err("Length of key is invalid. Use 1024, 2048, 3072 o 4096".into()),
    };
}