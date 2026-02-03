use std::env;
use std::fs;

use submission::help_fun::get_size_string;
use tfhe::core_crypto::prelude::{Container, ContiguousEntityContainer, LweCiphertext, LweCiphertextList, LweSecretKey, UnsignedInteger, decrypt_lwe_ciphertext};

pub fn decrypt_decode_lwe_list(
    lwe_sk: &LweSecretKey<Vec<u64>>,
    ciphertext: &LweCiphertextList<Vec<u64>>,
) -> Vec<u64> {
    let delta = 1_u64 << 63;
    let mut result: Vec<u64> = Vec::new();
    for c in ciphertext.iter() {
        result.push(decrypt(&lwe_sk, &c, delta));
    }
    result
}

fn decrypt<C, KeyCont, Scalar>(
    lwe_sk: &LweSecretKey<KeyCont>,
    lwe_ctxt: &LweCiphertext<C>,
    delta: Scalar,
) -> Scalar
where
    Scalar: UnsignedInteger,
    KeyCont: Container<Element = Scalar>,
    C: Container<Element = Scalar>,
{
    let scaling = lwe_ctxt
        .ciphertext_modulus()
        .get_power_of_two_scaling_to_native_torus();

    let decrypted = decrypt_lwe_ciphertext(&lwe_sk, &lwe_ctxt).0;
    let decrypted = decrypted * scaling;
    let rounding = (decrypted & (delta >> 1)) << 1;
    let decoded = (decrypted.wrapping_add(rounding)) / delta;
    return decoded;
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <size>", args[0]);
        std::process::exit(1); 
    }
    let size = args[1].clone();
    let io_dir = "io/".to_owned() + get_size_string(size.parse::<usize>()?);

    // Load secret key
    let secret_keys_dir = format!("{}/secret_keys", io_dir);
    let lwe_sk_path = format!("{}/lwe_sk.bin", secret_keys_dir);
    let lwe_sk_bytes = fs::read(&lwe_sk_path)?;
    let lwe_sk: LweSecretKey<Vec<u64>> = bincode::deserialize(&lwe_sk_bytes)?;

    // Load encrypted result from ciphertexts_download
    let ciphertexts_download_dir = format!("{}/ciphertexts_download", io_dir);
    let result_path = format!("{}/result.bin", ciphertexts_download_dir);
    let result_bytes = fs::read(&result_path)?;
    let lwe_ciphertext_list: LweCiphertextList<Vec<u64>> = bincode::deserialize(&result_bytes)?;

    // Decrypt and decode
    let decrypted_result = decrypt_decode_lwe_list(&lwe_sk, &lwe_ciphertext_list);

    // Save result to file
    let output_path = format!("{}/result.txt", io_dir);
    let result_str = decrypted_result
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join("");
    fs::write(&output_path, result_str)?;


    Ok(())
}