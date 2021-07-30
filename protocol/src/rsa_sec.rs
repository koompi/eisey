use pem::{EncodeConfig, LineEnding};
use rand_core::OsRng;
use rsa::{
    PaddingScheme, PrivateKeyPemEncoding, PublicKey, PublicKeyPemEncoding, RSAPrivateKey,
    RSAPublicKey,
};
use std::{fs::File, io::prelude::*, result::Result};

pub fn rsa_decrypt(enc_data: Vec<u8>) -> Result<Vec<u8>, rsa::errors::Error> {
    let mut bufs: String = String::new();
    let mut private_key_file = File::open("user_private.pem").unwrap();
    private_key_file.read_to_string(&mut bufs).unwrap();
    let der_encoded =
        bufs.lines()
            .filter(|line| !line.starts_with("-"))
            .fold(String::new(), |mut data, line| {
                data.push_str(&line);
                data
            });
    let der_bytes = base64::decode(&der_encoded).expect("failed to decode base64 content");
    let private_key = RSAPrivateKey::from_pkcs8(&der_bytes).unwrap();

    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    let dec_data = private_key.decrypt(padding, &enc_data);

    match dec_data {
        Ok(d) => Ok(d),
        Err(e) => Err(e),
    }
}

pub fn rsa_encrypt(data: String) -> Result<Vec<u8>, rsa::errors::Error> {
    let mut rng = OsRng;
    let mut bufs = Vec::<u8>::new();
    let mut public_key_file = File::open("user_public.pem").unwrap();
    public_key_file.read_to_end(&mut bufs).unwrap();
    let public_key = RSAPublicKey::from_pkcs8(&bufs).unwrap();

    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    let enc_data = public_key.encrypt(&mut rng, padding, data.as_bytes());

    match enc_data {
        Ok(d) => Ok(d),
        Err(e) => Err(e),
    }
}

pub fn rsa_keygen() {
    let mut rng = OsRng;

    let bits = 1024;
    let private_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RSAPublicKey::from(&private_key);

    let private_key_string = private_key.to_pem_pkcs8_with_config(EncodeConfig {
        line_ending: LineEnding::CRLF,
    });
    let public_key_string = public_key.to_pem_pkcs8_with_config(EncodeConfig {
        line_ending: LineEnding::CRLF,
    });

    let mut private_key_file = File::create("user_private.pem").unwrap();
    private_key_file
        .write_all(private_key_string.unwrap().as_bytes())
        .unwrap();
    let mut public_key_file = File::create("user_public.pem").unwrap();
    public_key_file
        .write_all(public_key_string.unwrap().as_bytes())
        .unwrap();
}
