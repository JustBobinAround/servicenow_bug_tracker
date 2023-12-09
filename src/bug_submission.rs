use openai_api::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_env_crypt::prelude::*;
 
pub struct BugSub {
    summary: String,
    environment: String,
    steps_to_reproduce: String,
    expected_behavior: String,
    actual_behavior: String,
    error_messages: String,
    additional_information: String,
}


impl BugSub {
    pub fn to_string(&self) -> String{
        let out = format!("Summary: \n{}", self.summary);
        let out = format!("{}\nEnvironment: \n{}",out, self.environment);
        let out = format!("{}\nSteps to Reproduce: \n{}",out, self.steps_to_reproduce);
        let out = format!("{}\nExpected Behavior: \n{}",out, self.expected_behavior);
        let out = format!("{}\nActual Behavior: \n{}",out, self.actual_behavior);
        let out = format!("{}\nError Messages: \n{}",out, self.error_messages);
        format!("{}\nAdditional Information: \n{}",out, self.additional_information)
    }

    pub fn split_by_bold_titles(input: &str) -> BugSub {
        let mut bug_sub = BugSub {
            summary: String::new(),
            environment: String::new(),
            steps_to_reproduce: String::new(),
            expected_behavior: String::new(),
            actual_behavior: String::new(),
            error_messages: String::new(),
            additional_information: String::new(),
        };

        let mut current_section = String::new();

        // Iterate over each line in the input text
        for line in input.lines() {
            // Check if the line starts with "**" to identify bold titles
            if line.starts_with("**Summary:") {
                // If there is content in the current section, assign it to the corresponding field in the struct
            } else if line.starts_with("**Environment:") {
                if !current_section.is_empty() {
                    let sanatized = current_section.replace("**Summary:**", "");
                    bug_sub.summary = sanatized.trim().to_string();
                    current_section.clear();
                }
            } else if line.starts_with("**Steps to Reproduce:") {
                if !current_section.is_empty() {
                    let sanatized = current_section.replace("**Environment:**", "");
                    bug_sub.environment = sanatized.trim().to_string();
                    current_section.clear();
                }
            } else if line.starts_with("**Expected Behavior:") {
                if !current_section.is_empty() {
                    let sanatized = current_section.replace("**Steps to Reproduce:**", "");
                    bug_sub.steps_to_reproduce = sanatized.trim().to_string();
                    current_section.clear();
                }
            } else if line.starts_with("**Actual Behavior:") {
                if !current_section.is_empty() {
                    let sanatized = current_section.replace("**Expected Behavior:**", "");
                    bug_sub.expected_behavior = sanatized.trim().to_string();
                    current_section.clear();
                }
            } else if line.starts_with("**Error Messages:") {
                if !current_section.is_empty() {
                    let sanatized = current_section.replace("**Actual Behavior:**", "");
                    bug_sub.actual_behavior = sanatized.trim().to_string();
                    current_section.clear();
                }
            } else if line.starts_with("**Additional Information:") {
                if !current_section.is_empty() {
                    let sanatized = current_section.replace("**Error Messages:**", "");
                    bug_sub.error_messages = sanatized.trim().to_string();
                    current_section.clear();
                }
            }

            current_section.push_str(line);
            current_section.push('\n');
        }
        let sanatized = current_section.replace("**Additional Information:**", "");
        bug_sub.additional_information = sanatized.trim().to_string();
        current_section.clear();

        bug_sub
    }

    pub async fn gpt_autofill() -> Option<BugSub> {
        let request = gpt35![
            system!("make up a bug report following the markdown template provided by the user:"),
            user!(r#"**Summary:**

**Environment:**

**Steps to Reproduce:**

**Expected Behavior:**

**Actual Behavior:**

**Error Messages:**

**Additional Information:**
"#),
        ].get().await;
        match request {
            Ok(response) => {
                let input_text = response.default_choice();
                Some(BugSub::split_by_bold_titles(&input_text))
            },
            Err(_) => {None}
        }
    }
}

#[wasm_bindgen]
pub async fn autofill_form(pass: String) -> Result<JsValue, JsValue>{
    if check_passcode(&pass, super::HASH) {
        let document = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?.document().ok_or_else(|| JsValue::from_str("No document"))?;

        let key = xor_decrypt(&super::OPENAI_API_KEY, &pass);
        openai_api::key::set_api_key(key);

        let bug_sub = BugSub::gpt_autofill().await;

        if let Some(bug_sub) = bug_sub {
            let element = document.get_element_by_id("summary").ok_or_else(|| JsValue::from_str("Element not found"))?;
            element.set_inner_html(&bug_sub.summary);
            let element = document.get_element_by_id("environment").ok_or_else(|| JsValue::from_str("Element not found"))?;
            element.set_inner_html(&bug_sub.environment);
            let element = document.get_element_by_id("stepsToReproduce").ok_or_else(|| JsValue::from_str("Element not found"))?;
            element.set_inner_html(&bug_sub.steps_to_reproduce);
            let element = document.get_element_by_id("expectedBehavior").ok_or_else(|| JsValue::from_str("Element not found"))?;
            element.set_inner_html(&bug_sub.expected_behavior);
            let element = document.get_element_by_id("actualBehavior").ok_or_else(|| JsValue::from_str("Element not found"))?;
            element.set_inner_html(&bug_sub.actual_behavior);
            let element = document.get_element_by_id("errorMessages").ok_or_else(|| JsValue::from_str("Element not found"))?;
            element.set_inner_html(&bug_sub.error_messages);
            let element = document.get_element_by_id("additionalInformation").ok_or_else(|| JsValue::from_str("Element not found"))?;
            element.set_inner_html(&bug_sub.additional_information);
        } else {
            return Err(JsValue::FALSE);
        }
    }    
    Ok(JsValue::TRUE)
}
