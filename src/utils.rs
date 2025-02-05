use std::{env, fs, io};
use dotenv::dotenv;
use google_generative_ai_rs::v1::gemini::request::{GenerationConfig, SystemInstructionContent, SystemInstructionPart};
use google_generative_ai_rs::v1::gemini::response::{self, GeminiResponse};
use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{request::Request, Content, Model, Part, Role},
};

pub fn configure_generation_model(model_name: String) -> Model {
    match model_name.as_str() {
        "Gemini1_0Pro" => Model::Gemini1_0Pro,
        "Gemini1_5Pro" => Model::Gemini1_5Pro,
        "Gemini1_5Flash" => Model::Gemini1_5Flash,
        "Gemini1_5Flash8B" => Model::Gemini1_5Flash8B,
        "Gemini2_0Flash" => Model::Gemini2_0Flash,
        _ => Model::Gemini1_5Pro,
    }
}
