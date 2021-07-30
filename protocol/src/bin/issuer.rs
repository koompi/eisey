use bip32::{Error, ExtendedPrivateKey, ExtendedPublicKey, Language};
use bip32::{Mnemonic, Prefix, XPrv};
use k256::ecdsa::signature::Signature;
use k256::ecdsa::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use rsa::{
    PaddingScheme, PrivateKeyPemEncoding, PublicKey, PublicKeyPemEncoding, RSAPrivateKey,
    RSAPublicKey,
};
#[allow(unused_imports)]
use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::result::Result;
use std::str::from_utf8;
use std::str::FromStr;
fn main() {
    let args: Vec<String> = env::args().skip(1).map(|arg| arg.to_string()).collect();

    match args[0].as_ref() {
        "keygen" => genkey().unwrap(),
        "sign" => {
            let input = args[1].clone();
            let user_pub = args[2].clone();
            let enc_input = rsa_encrypt(input, &user_pub).unwrap();
            let enc_str = base64::encode_config(enc_input.clone(), base64::URL_SAFE_NO_PAD);
            let sigb = sign_byte(&enc_input);
            let sigb_str = base64::encode_config(sigb, base64::URL_SAFE_NO_PAD);

            println!("koompi://init?cmd={}&cert={}", &enc_str, &sigb_str);
        }
        _ => {}
    }
}

pub fn rsa_encrypt(data: String, pubkey: &str) -> Result<Vec<u8>, rsa::errors::Error> {
    let mut rng = OsRng;
    let mut bufs: String = String::new();
    let mut public_key_file = File::open(pubkey).unwrap();
    public_key_file.read_to_string(&mut bufs).unwrap();
    let der_encoded =
        bufs.lines()
            .filter(|line| !line.starts_with("-"))
            .fold(String::new(), |mut data, line| {
                data.push_str(&line);
                data
            });
    let der_bytes = base64::decode(&der_encoded).expect("failed to decode base64 content");
    let public_key = RSAPublicKey::from_pkcs8(&der_bytes).unwrap();

    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    let enc_data = public_key.encrypt(&mut rng, padding, data.as_bytes());

    match enc_data {
        Ok(d) => Ok(d),
        Err(e) => Err(e),
    }
}

pub fn genkey() -> Result<(), Error> {
    let mnemonic= Mnemonic::new("analyst lonely nation often clay renew lazy tell siren antenna define double hole ancient couple solid inch fix charge leisure belt price art pole", Language::English).unwrap();
    let seed = mnemonic.to_seed("fuckyou");

    let child_path = "m/0/2147483647'/1/2147483646'";
    let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?;
    let child_xpub = child_xprv.public_key();

    let child_xprv_str = child_xprv.to_string(Prefix::XPRV);

    let child_xpub_str = child_xpub.to_string(Prefix::XPUB);

    println!("{}", child_xprv_str.to_string());
    println!("{}", child_xpub_str.to_string());

    let mut prvf = File::create("prv.key").unwrap();
    let mut pubf = File::create("pub.key").unwrap();

    prvf.write_all(child_xprv_str.as_bytes()).unwrap();
    pubf.write_all(child_xpub_str.as_bytes()).unwrap();

    Ok(())
}

pub fn sign(input: &str, cert_name: &str) {
    let mut prv_str = String::new();
    let mut prv_file = File::open("prv.key").unwrap();
    prv_file.read_to_string(&mut prv_str).unwrap();

    use bip32::secp256k1::ecdsa::{signature::Signer, Signature};

    let prv: ExtendedPrivateKey<SigningKey> = ExtendedPrivateKey::from_str(&prv_str).unwrap();
    let signature: Signature = prv.private_key().sign(&input.as_bytes());
    let sig_string = signature.as_bytes();
    let mut sig_file = File::create(cert_name).unwrap();
    sig_file.write_all(sig_string).unwrap();
}

pub fn sign_byte(input: &[u8]) -> Vec<u8> {
    let mut prv_str = String::new();
    let mut prv_file = File::open("prv.key").unwrap();
    prv_file.read_to_string(&mut prv_str).unwrap();

    use bip32::secp256k1::ecdsa::{signature::Signer, Signature};

    let prv: ExtendedPrivateKey<SigningKey> = ExtendedPrivateKey::from_str(&prv_str).unwrap();
    let signature: Signature = prv.private_key().sign(&input);
    let sig_string = signature.as_bytes();
    sig_string.to_vec()
    // let mut sig_file = File::create(cert_name).unwrap();
    // sig_file.write_all(sig_string).unwrap();
}

pub fn verify(singnature: &str, input: &str) -> Result<(), Error> {
    let mut buffer = Vec::<u8>::new();
    let mut sig_file1 = File::open(singnature).unwrap();
    sig_file1.read_to_end(&mut buffer).unwrap();
    use bip32::secp256k1::ecdsa::{signature::Verifier, Signature};
    let new_sig: Signature = Signature::from_bytes(&buffer).unwrap();
    let mut pubkey_string = String::new();
    let mut pubkey_file = File::open("pub.key").unwrap();
    pubkey_file.read_to_string(&mut pubkey_string).unwrap();
    let pubkey: ExtendedPublicKey<VerifyingKey> =
        ExtendedPublicKey::from_str(&pubkey_string).unwrap();

    if pubkey
        .public_key()
        .verify(&input.as_bytes(), &new_sig)
        .is_ok()
    {
        println!("Success")
    } else {
        println!("Failed")
    }
    Ok(())
}
