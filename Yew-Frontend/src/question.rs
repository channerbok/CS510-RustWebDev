use crate::*;
use wasm_bindgen::prelude::*;

// Data type
#[derive(Debug, Properties, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct QuestionStruct {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub answer: String,
    pub tags: Option<HashSet<String>>,
    pub source: Option<String>,
}

// Error Logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log_1(s: &str);
}

// Grabs Question from backend application
impl QuestionStruct {
    pub async fn get_question(key: Option<String>) -> Msg {
        let host = "http://localhost:8000";
        let request = match &key {
            None => {
                format!("{}/api/v1/question", host)
            }
            Some(ref key) => {
                format!("{}/api/v1/question/{}", host, key)
            }
        };

        let response = http::Request::get(&request).send().await;

        match response {
            Err(e) => Msg::GotQuestion(Err(e)),
            Ok(data) => {
                let json_data = data.json().await;
                match &json_data {
                    Ok(_json) => {}
                    Err(_e) => {}
                }
                Msg::GotQuestion(json_data)
            }
        }
    }
}

// Formats tag hash
pub fn format_tags2(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

// Secondary datatype for formatting
#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionProps {
    pub question: QuestionStruct,
}

#[function_component(Question)]
pub fn question(question: &QuestionProps) -> Html {
    let question = &question.question;
    html! { <>
        <div class="question">
            <span class="category">{format!("{} Based Question:", question.content.clone())}</span><br/>
            <span class="title">{question.title.clone()}</span><br/>
            <span class="title">{format!("Answer:")}</span><br/>
            <span class="answer">{question.answer.clone()}</span>
        </div>
        <span class="annotation">
            {format!("[id: {}", &question.id)}
            if let Some(ref tags) = question.tags {
                {format!("; tags: {}", &format_tags2(tags))}
            }
            if let Some(ref source) = question.source {
                {format!("; source: {}", source)}
            }
            {"]"}
        </span>
    </> }
}
