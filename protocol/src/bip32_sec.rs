use bip32::{Error, ExtendedPrivateKey, ExtendedPublicKey, Language};
use bip32::{Mnemonic, Prefix, XPrv};
use k256::ecdsa::signature::Signature;
use k256::ecdsa::{SigningKey, VerifyingKey};
#[allow(unused_imports)]
use rand_core::OsRng;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::str::FromStr;

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

pub fn verify(singnature: &str, input: &str) -> Result<(), Error> {
    // read signature file
    let mut buffer = Vec::<u8>::new();
    let mut sig_file1 = File::open(singnature).unwrap();
    sig_file1.read_to_end(&mut buffer).unwrap();
    use bip32::secp256k1::ecdsa::{signature::Verifier, Signature};
    let new_sig: Signature = Signature::from_bytes(&buffer).unwrap();
    // read public key
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

pub fn verify_bytes(singnature: &[u8], input: &[u8]) -> Result<(), Error> {
    use bip32::secp256k1::ecdsa::{signature::Verifier, Signature};
    let new_sig: Signature = Signature::from_bytes(singnature).unwrap();
    // read public key
    let mut pubkey_string = String::new();
    #[cfg(debug_assertions)]
    let mut pubkey_file = File::open("pub.key").unwrap();
    #[cfg(not(debug_assertions))]
    let mut pubkey_file = File::open("/usr/share/org.koompi.sel/pub.key").unwrap();
    pubkey_file.read_to_string(&mut pubkey_string).unwrap();
    let pubkey: ExtendedPublicKey<VerifyingKey> =
        ExtendedPublicKey::from_str(&pubkey_string).unwrap();

    if pubkey.public_key().verify(&input, &new_sig).is_ok() {
        println!("Success")
    } else {
        println!("Failed")
    }
    // println!("{:?}", pubkey);
    // assert_eq!(pubkey.to_string(Prefix::XPUB), pubkey_string);

    Ok(())
}
