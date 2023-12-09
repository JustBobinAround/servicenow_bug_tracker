use openai_api::prelude::*;
use wasm_bindgen::prelude::*;

use wasm_env_crypt::prelude::*;

wasm_env_crypt::load_crypt_env!();

pub mod bug_submission;



#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

macro_rules! wasm_crypt {
    () => {
        let wasm_crypt = env!("WASM_CRYPT", "Failed to get WASM_CRYPT env variable at build");
        let api_key = env!("OPENAI_API_KEY", "Failed to get OPENAI_API_KEY env variable, at build");

        let encrypted_api_key = xor_encrypt(&api_key, &wasm_crypt);
        set_api_key(xor_encrypt(&encrypted_api_key, wasm_crypt))
    };
}

#[wasm_bindgen]
pub fn is_passcode_correct(pass: String) -> bool {
    check_passcode(&pass, HASH)
}

