import init, { is_passcode_correct } from "../wasm/servicenow_bug_tracker.js";
import { cookie_exists, set_cookie, get_cookie } from "./cookies.js";
export function check_wasm() {
    document.body.style.display = "none";
    if (!(typeof WebAssembly === "object" && typeof WebAssembly.instantiate === "function")) {
        window.location.href="../index.html";
    }
}

export function auth(to_exe) {
    init().then(() => {
        if(cookie_exists("service_now_demo")) {
            const passcode = get_cookie("service_now_demo");
            if (is_passcode_correct(passcode)) {
                document.body.style.display = "block";
                to_exe(passcode);
            } else {
                window.location.href="../index.html";
            }
        } else {
            window.location.href="../index.html";
        }
    });
}

