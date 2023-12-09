pub mod bug_submission;

use wasm_bindgen::prelude::*;
use wasm_env_crypt::prelude::*;

wasm_env_crypt::load_crypt_env!();

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn is_passcode_correct(pass: String) -> bool {
    check_passcode(&pass, HASH)
}

