use openai_api::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_env_crypt::prelude::*;
use reqwest::{Client, Response};
use serde::{self, Serialize, Deserialize};

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
            let text = element.text_content().ok_or_else(|| JsValue::from_str("Failed to get inner text"))?;
            text
        }
    };

}

#[derive(Serialize, Deserialize, Debug)]
pub struct TabelRequest {
    result: Vec<BugSub>
}

impl TabelRequest {
    pub async fn get_table(pass:String, amount: u32) -> Option<TabelRequest> {
        let url = format!("https://dev215866.service-now.com/api/now/table/x_1156972_bug_tr_0_bug_table?sysparm_limit={}", amount);
        let username = xor_decrypt(&super::SERVICENOW_USER, &pass);
        let pass = xor_decrypt(&super::SERVICENOW_PASS, &pass);
        let client = Client::new();
        let response = client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .basic_auth(username, Some(pass))
            .send().await;

        match response {
            Ok(response) => {
                let response: Result<TabelRequest, reqwest::Error> = response.json().await;
                match response {
                    Ok(bug_sub) => {
                        Some(bug_sub)
                    },
                    Err(_) => {
                        None
                    }
                }
            },
            Err(_) => {
                None
            }
        }

    }
    pub async fn populate_table(&self) -> Result<JsValue, JsValue>{
        for item in &self.result {
            if item.add_to_table().is_err() {
                return Err(JsValue::from_str("failed to add item to table"));
            }
        }
        Ok(JsValue::from_str(&format!("{}",self.result.len())))
    }
}
 
#[derive(Serialize, Deserialize, Debug)]
pub struct BugSub {
    actual_behavior: String,
    additional_information: String,
    assigned_to: String,
    description: String,
    environment: String,
    error_messages: String,
    expected_behavior: String,
    #[serde(skip_serializing)]
    number: String,
    recommend_user_actions: String,
    steps_to_reproduce: String,
    summary: String,
    severity: String,
    #[serde(skip_serializing)]
    sys_id: String,
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

    pub fn add_to_table(&self) -> Result<(), JsValue> {
        let document = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?.document().ok_or_else(|| JsValue::from_str("No document"))?;
        let element = document.get_element_by_id("bug-tracker-body").ok_or_else(|| JsValue::from_str("Element not found"))?;
        let tr = document.create_element("tr")?;
        let td_id = document.create_element("td")?;
        let a_id = document.create_element("a")?;
        let td_title = document.create_element("td")?;
        let td_desc = document.create_element("td")?;
        let td_assigned_to = document.create_element("td")?;
        let td_severity = document.create_element("td")?;

        td_id.set_class_name("important-cell");
        a_id.set_attribute("href", &format!("./pages/bug_item.html#{}", self.sys_id))?;
        a_id.set_text_content(Some(&self.number));
        td_id.append_child(&a_id)?;
        td_title.set_class_name("important-cell");
        td_title.set_text_content(Some(&self.title));
        td_desc.set_text_content(Some(&self.description));
        td_assigned_to.set_text_content(Some(&self.assigned_to));
        td_severity.set_text_content(Some(&self.severity));

        tr.append_child(&td_id)?;
        tr.append_child(&td_title)?;
        tr.append_child(&td_desc)?;
        tr.append_child(&td_assigned_to)?;
        tr.append_child(&td_severity)?;

        element.append_child(&tr)?;

        Ok(())
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
            number: String::new(),
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

    pub async fn send_to_table(&self, pass: String) {
        let url = "https://dev215866.service-now.com/api/now/table/x_1156972_bug_tr_0_bug_table";
        let username = xor_decrypt(&super::SERVICENOW_USER, &pass);
        let pass = xor_decrypt(&super::SERVICENOW_PASS, &pass);
        let client = Client::new();
        let response = client
            .post(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .basic_auth(username, Some(pass))
            .json(&self) // Serialize the JSON body
            .send().await;
    }
}

#[wasm_bindgen]
pub async fn build_bugsub(pass: String) -> Result<JsValue, JsValue>{
    let mut bug_sub = BugSub {
        actual_behavior: get_text_by_id!("actualBehavior"),
        additional_information: get_text_by_id!("additionalInformation"),
        assigned_to: String::new(),
        description: String::new(),
        environment: get_text_by_id!("environment"),
        error_messages: get_text_by_id!("errorMessages"),
        expected_behavior: get_text_by_id!("expectedBehavior"),
        number: String::from("1"),
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
    bug_sub.send_to_table(pass).await;

    bug_sub.add_to_table()?;

    let test = format!("Recommended Actions:\n{}\nTitle:\n{}\nDesc:\n{}\nSeverity:\n{}\nResponse:", &bug_sub.recommend_user_actions, &bug_sub.title,
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

#[wasm_bindgen]
pub async fn fill_table(pass: String) -> Result<JsValue, JsValue> {
    let mut table_len = JsValue::from_str("");
    if check_passcode(&pass, super::HASH) {
        if let Some(table) = TabelRequest::get_table(pass, 100).await {
            table_len = table.populate_table().await?;
        } else {
            table_len = JsValue::from_str("failed to parse table");
        }
    }
    Ok(table_len)
}
