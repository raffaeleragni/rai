use llm::models::Llama;
use llm::KnownModel;
use std::convert::Infallible;
use std::env;
use std::path::PathBuf;

pub struct Rai {
    model: Llama,
    purpose: String,
    pub conversation: Conversation,
}

impl Default for Rai {
    fn default() -> Self {
        Rai::from_purpose("Your purpose is to assist in answering Human.")
    }
}

impl Rai {
    pub fn from_purpose(purpose: &str) -> Self {
        let mut rai = Rai {
            model: get_language_model(),
            purpose: purpose.into(),
            conversation: Conversation::new(),
        };
        rai.recompute();
        rai
    }

    pub fn prompt(&mut self, prompt: &str) {
        self.conversation.messages.push(Message {
            user: true,
            text: prompt.into(),
        });
        self.recompute();
    }

    pub fn recompute(&mut self) {
        let character_name = "AI";
        let user_name = "Human";
        let persona = "A chat between a Human and an AI.";
        let purpose = self.purpose.clone();
        let mut history = format!("{user_name}:{purpose}\n");

        for message in self.conversation.messages.iter() {
            let msg = message.text.clone();
            let curr_line = if message.user {
                format!("{character_name}:{msg}\n")
            } else {
                format!("{user_name}:{msg}\n")
            };

            history.push_str(&curr_line);
        }

        let mut res = String::new();
        let mut rng = rand::thread_rng();
        let mut buf = String::new();

        let mut session = self.model.start_session(Default::default());
        session
            .infer(
                &self.model,
                &mut rng,
                &llm::InferenceRequest {
                    prompt: format!("{persona}\n{history}\n{character_name}:")
                        .as_str()
                        .into(),
                    parameters: &llm::InferenceParameters::default(),
                    play_back_previous_tokens: false,
                    maximum_token_count: None,
                },
                &mut Default::default(),
                inference_callback(String::from(user_name), &mut buf, &mut res),
            )
            .unwrap_or_else(|e| panic!("{e}"));

        self.conversation.messages.push(Message {
            user: false,
            text: res,
        });
    }
}

#[derive(Debug)]
pub struct Conversation {
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Conversation {
            messages: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub user: bool,
    pub text: String,
}

fn inference_callback<'a>(
    stop_sequence: String,
    buf: &'a mut String,
    out_str: &'a mut String,
) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
    use llm::InferenceFeedback::Continue;
    use llm::InferenceFeedback::Halt;

    move |resp| match resp {
        llm::InferenceResponse::InferredToken(t) => {
            let mut reverse_buf = buf.clone();
            reverse_buf.push_str(t.as_str());
            if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                buf.clear();
                return Ok::<llm::InferenceFeedback, Infallible>(Halt);
            } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                buf.push_str(t.as_str());
                return Ok(Continue);
            }

            if buf.is_empty() {
                out_str.push_str(&t);
            } else {
                out_str.push_str(&reverse_buf);
            }

            Ok(Continue)
        }
        llm::InferenceResponse::EotToken => Ok(Halt),
        _ => Ok(Continue),
    }
}

fn get_language_model() -> Llama {
    dotenv::dotenv().ok();
    let model_path = env::var("MODEL_PATH").expect("MODEL_PATH must be set");

    llm::load::<Llama>(
        &PathBuf::from(&model_path),
        llm::TokenizerSource::Embedded,
        Default::default(),
        llm::load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to laod model from {model_path:?}: {err}"))
}
