mod question;

use question::*;
use std::collections::HashSet;

use gloo_net::http;

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

pub type QuestionResult = Result<QuestionStruct, gloo_net::Error>;

struct App {
    question: QuestionResult,
    answer_input: String,
}

// Define the possible messages (events) that the application can handle
pub enum Msg {
    GotQuestion(QuestionResult),
    GetQuestion(Option<String>),
    UpdateAnswer(String),
    SubmitAnswer,
}

impl App {
    // Function to refresh the question by sending a future request to get a question
    fn refresh_question(ctx: &Context<Self>, key: Option<String>) {
        let future = async move { QuestionStruct::get_question(key).await };
        ctx.link().send_future(future);
    }
}
#[allow(unused_variables)]
impl Component for App {
    type Message = Msg;
    type Properties = ();

    // Function to create the initial state of the component
    fn create(ctx: &Context<Self>) -> Self {
        App::refresh_question(ctx, None);
        let question = Err(gloo_net::Error::GlooError("Loading Question…".to_string()));
        Self {
            question,
            answer_input: String::new(),
        }
    }

    // Function to update the component state based on received messages
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotQuestion(question) => {
                self.question = question;
                true
            }
            Msg::GetQuestion(key) => {
                App::refresh_question(ctx, key);
                false
            }
            Msg::UpdateAnswer(input) => {
                self.answer_input = input;
                false
            }
            Msg::SubmitAnswer => {
                if let Ok(ref mut question) = self.question {
                    question.answer = self.answer_input.clone();
                }
                true
            }
        }
    }

    // Function to define the component's view
    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput_answer = ctx.link().callback(|e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            Msg::UpdateAnswer(input.value())
        });

        let question = &self.question;

        // Display Formatting
        html! {
            <>
                <h1 class="header">{ "Questions and Answers!" }</h1>

                <div class="margin-bottom-10">
                    {match &self.question {
                        Ok(question) => html!{ <Question question={question.clone()} /> },
                        Err(error) => html!{ <div><span class="error">{format!("Server Error: {}", error)}</span></div> },
                    }}
                </div>
                <div class="button">
                    <button onclick={ctx.link().callback(|_| Msg::GetQuestion(None))} class="button">{"Give me a question!"}</button>
                </div>

                <div class="box">
                    <textarea placeholder="Enter your answer here" oninput={oninput_answer.clone()} class="box align-middle"></textarea>
                    <button onclick={ctx.link().callback(|_| Msg::SubmitAnswer)}class="align-middle"> {"Submit Answer"}</button>
                </div>
            </>
        }
    }
}

// Main function to render the `App` component

fn main() {
    yew::Renderer::<App>::new().render();
}
