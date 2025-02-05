use std::{env, fs, io};
use google_generative_ai_rs::v1::gemini::request::{GenerationConfig, SystemInstructionContent, SystemInstructionPart};
use google_generative_ai_rs::v1::gemini::response::{self, GeminiResponse};
use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{request::Request, Content, Model, Part, Role},
};

/// JSON-based text request using the public API and an API key for authn
///
/// NOTE: Currently, only available on the v1beta API.
///
/// To run:
/// ```
/// API_KEY=[YOUR_API_KEY] RUST_LOG=info cargo run -- features "beta" --package google-generative-ai-rs  --example text_request_json
/// ``
#[tokio::main]
async fn generate_rules(
    system_instruction_path: String,
    form_respond_path: String,
    form_question_path: String,
    output_path: String,
    model_name: String,
    generation_config: 
    
    ) -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    #[cfg(not(feature = "beta"))]
    {
        log::error!("JSON-mode only works currently on Gemini 1.5 Pro and on 'beta'");

        Ok(())
    }

    #[cfg(feature = "beta")]
    {
        // Either run as a standard text request or a stream generate content request
        let client: google_generative_ai_rs::v1::api::Client = Client::new_from_model(
            Model::Gemini1_5Pro,
            "AIzaSyBFaMkSGEWx2CCEt_VzqzjLhXfX9eKd3SQ".to_string()
            );

        let system_instruction_path="./src/system_prompt.md";
        let form_question_path="./src/form_question.json";
        let form_respond_path="./src/form_respond.json";
        let system_instruction_contents = fs::read_to_string(system_instruction_path)?;
        let form_question_contents = fs::read_to_string(form_question_path)?;
        let form_respond_contents = fs::read_to_string(form_respond_path)?;

        let prompt = format!("form_question: {}\nform_respond: {}", form_question_contents, form_respond_contents);
        log::info!("Prompt: {:#?}", prompt);

        let txt_request = Request {
            contents: vec![Content {
                role: Role::User,
                parts: vec![Part {
                    text: Some(prompt),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            }],
            tools: vec![],
            safety_settings: vec![],
            generation_config: Some(GenerationConfig {
                temperature: Some(1.0),
                top_p: Some(0.9),
                top_k: Some(64),
                candidate_count: None,
                max_output_tokens: Some(32768),
                stop_sequences: None,
                response_mime_type: Some("application/json".to_string()),
                response_schema: None,
            }),

            system_instruction: Some(SystemInstructionContent {
                parts: vec![SystemInstructionPart{
                    text: Some(system_instruction_contents),
                }],
            }),
        };

        let response = client.post(30, &txt_request).await?;
        println!("response: {}",response.rest().unwrap().candidates[0].content.parts[0].text.as_ref().unwrap());
        //log::info!("{:#?}", response);

        Ok(())
    }
}