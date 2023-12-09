use wasm_env_crypt::prelude::*;

fn main() {
    set_crypt_env(vec!["OPENAI_API_KEY", "SERVICENOW_USER", "SERVICENOW_PASS"]);
}

