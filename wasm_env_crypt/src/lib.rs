pub mod prelude {
    use std::env;
    use sha2::{Digest, Sha512};
    pub fn set_crypt_env(names: Vec<&str>) {
        println!("cargo:rerun-if-env-changed=WASM_ENV_CRYPT");
        let wasm_crypt = env::var("WASM_ENV_CRYPT")
            .expect("Failed to get WASM_ENV_CRYPT env variable");
        let mut out: String = String::from("//THESE ARE ENCRYPTED\n");
        let mut hasher = Sha512::new();
        hasher.update(wasm_crypt.clone());
        let crypted_var = format!("const HASH: &str = \"{:x}\";\n", hasher.finalize());
        out.push_str(&crypted_var);
        for name in names {
            println!("cargo:rerun-if-env-changed={}", name);
            let api_key = env::var(&name)
                .expect(&format!("Failed to get {} env variable",name));
            let encrypted_api_key = xor_encrypt(&api_key, &wasm_crypt);
            let encrypted_api_key_len = encrypted_api_key.len();
            let mut bits = String::new();
            for bit in encrypted_api_key {
                bits.push_str(&bit.to_string());
                bits.push(',');
            }
            let crypted_var = format!("const {}: [u8;{}] = [{}];\n", &name, encrypted_api_key_len, bits);
            out.push_str(&crypted_var);
        }
        let dest_path = std::path::Path::new("wasm_crypt_vars.rs");
        std::fs::write(
            &dest_path,
            out
        ).expect("failed to write wasm_crypt_vars.rs");
    }

    pub fn xor_decrypt(input: &[u8], key: &str) -> String {
        let input: String = input
            .into_iter()
            .zip(key.chars().cycle()) 
            .map(|(c, k)| (c ^ k as u8) as char) 
            .collect();
        input.trim_end().to_string()
    }
    pub fn xor_encrypt(input: &str, key: &str) -> Vec<u8> {
        let mut input = input.to_string();
        if input.len()<40 {
            for i in (40-input.len())..=40 {
                input.push(' ');
            }
        }
        input
            .chars()
            .zip(key.chars().cycle()) 
            .map(|(c, k)| (c as u8 ^ k as u8)) 
            .collect()
    }

    pub fn check_passcode(key: &str, hash: &str) -> bool {
        let mut hasher = Sha512::new();
        hasher.update(key);
        hash==format!("{:x}", hasher.finalize())
    }


}
#[macro_export]
macro_rules! load_crypt_env {
    () => {
        include!("../wasm_crypt_vars.rs");
    };
}
