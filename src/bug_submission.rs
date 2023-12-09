use openai_api::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_env_crypt::prelude::*;
use tokio::time::Duration;

pub struct BugTableItem {
}
 
pub struct BugSub {
    actual_behavior: String,
    additional_information: String,
    assigned_to: String,
    description: String,
    environment: String,
    error_messages: String,
    expected_behavior: String,
    recommend_user_actions: String,
    steps_to_reproduce: String,
    summary: String,
    severity: String,
    title: String,
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


    pub async fn get_recommended_options(&mut self) {
        let bug_sub_str = self.to_string();
        let request = gpt35![
            system!("based on the following bug report, ENUMERATE a list of things the user can try while waiting for a solution. Provide ONLY the list of options."),
            user!(bug_sub_str),
        ].get().await;
        match request {
            Ok(response) => {
                self.recommend_user_actions = response.default_choice();
            },
            Err(_) => {}
        }
    }

    pub async fn get_severity(&mut self) {
        let bug_sub_str = self.to_string();
        let request = gpt35![
            system!("based on the following bug report, choose a severity of either LOW, MEDIUM, or HIGH. DO NOT give a breakdown, only provide a SINGLE WORD as an answer."),
            user!(bug_sub_str),
        ].get().await;
        match request {
            Ok(response) => {
                self.severity = response.default_choice();
            },
            Err(_) => {}
        }
    }
    
    pub async fn get_description(&mut self) {
        let bug_sub_str = self.to_string();
        let request = gpt35![
            system!("make up a VERY SHORT description for the following bug report:"),
            user!(bug_sub_str),
        ].get().await;
        match request {
            Ok(response) => {
                self.description = response.default_choice();
            },
            Err(_) => {}
        }
    }

    pub async fn get_title(&mut self) {
        let bug_sub_str = self.to_string();
        let request = gpt35![
            system!("make up a VERY SHORT title for the following bug report:"),
            user!(bug_sub_str),
        ].get().await;
        match request {
            Ok(response) => {
                self.title = response.default_choice();
            },
            Err(_) => {}
        }
    }

    pub fn load_from_autofill(input: &str) -> BugSub {
        let mut bug_sub = BugSub {
            actual_behavior: String::new(),
            additional_information: String::new(),
            assigned_to: String::new(),
            description: String::new(),
            environment: String::new(),
            error_messages: String::new(),
            expected_behavior: String::new(),
            recommend_user_actions: String::new(),
            steps_to_reproduce: String::new(),
            summary: String::new(),
            severity: String::new(),
            title: String::new(),
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
                Some(BugSub::load_from_autofill(&input_text))
            },
            Err(_) => {None}
        }
    }
}
macro_rules! set_text_by_id {
    ($name:literal, $content:expr) => {
        {
            let document = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?.document().ok_or_else(|| JsValue::from_str("No document"))?;
            let element = document.get_element_by_id($name).ok_or_else(|| JsValue::from_str("Element not found"))?;
            element.set_inner_html($content);
        }
    };
}

macro_rules! get_text_by_id {
    ($name:literal) => {
        {
            let document = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?.document().ok_or_else(|| JsValue::from_str("No document"))?;
            let element = document.get_element_by_id($name).ok_or_else(|| JsValue::from_str("Element not found"))?;
            element.text_content().ok_or_else(|| JsValue::from_str("Failed to get inner text"))?
        }
    };
}

#[wasm_bindgen]
pub async fn build_bugsub() -> Result<JsValue, JsValue>{
    let mut bug_sub = BugSub {
        actual_behavior: get_text_by_id!("actualBehavior"),
        additional_information: get_text_by_id!("additionalInformation"),
        assigned_to: String::new(),
        description: String::new(),
        environment: get_text_by_id!("environment"),
        error_messages: get_text_by_id!("errorMessages"),
        expected_behavior: get_text_by_id!("expectedBehavior"),
        recommend_user_actions: String::new(),
        steps_to_reproduce: get_text_by_id!("stepsToReproduce"),
        summary: get_text_by_id!("summary"),
        severity: String::new(),
        title: String::new(),
    };
    bug_sub.get_recommended_options().await;
    bug_sub.get_title().await;
    bug_sub.get_description().await;
    bug_sub.get_severity().await;

    let test = format!("Recommended Actions:\n{}\nTitle:\n{}\nDesc:\n{}\nSeverity:\n{}", &bug_sub.recommend_user_actions, &bug_sub.title,
                       &bug_sub.description, &bug_sub.severity);

    Ok(JsValue::from_str(&test))
}

#[wasm_bindgen]
pub async fn autofill_form(pass: String) -> Result<JsValue, JsValue>{
    if check_passcode(&pass, super::HASH) {
        let key = xor_decrypt(&super::OPENAI_API_KEY, &pass);
        openai_api::key::set_api_key(key);

        let bug_sub = BugSub::gpt_autofill().await;

        if let Some(bug_sub) = bug_sub {
            set_text_by_id!("summary", &bug_sub.summary);
            set_text_by_id!("environment", &bug_sub.environment);
            set_text_by_id!("stepsToReproduce", &bug_sub.steps_to_reproduce);
            set_text_by_id!("expectedBehavior", &bug_sub.expected_behavior);
            set_text_by_id!("actualBehavior", &bug_sub.actual_behavior);
            set_text_by_id!("errorMessages", &bug_sub.error_messages);
            set_text_by_id!("additionalInformation", &bug_sub.additional_information);
        } else {
            return Err(JsValue::FALSE);
        }
    }    
    Ok(JsValue::TRUE)
}
