use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::rngs::OsRng;
use rand::TryRngCore;
use sha3::{Digest, Sha3_512};
use std::{env, fs, path::PathBuf};

fn main() {
    if let Ok(tok) = env::var("APP_TOKEN") {
        println!("cargo:rustc-env=APP_TOKEN={tok}");
        println!("cargo:rerun-if-env-changed=APP_TOKEN");
        return;
    }

    let mut seed = [0u8; 32];
    OsRng.try_fill_bytes(&mut seed).expect("OS RNG unavailable");
    let digest = Sha3_512::digest(&seed);
    let token = URL_SAFE_NO_PAD.encode(&digest[..6]);

    println!("cargo:rustc-env=APP_TOKEN={token}");
    println!("cargo:rerun-if-env-changed=APP_TOKEN");

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let _ = fs::write(out.join("APP_TOKEN.txt"), &token);
}
