#![warn(clippy::cast_lossless)]

use clap::Subcommand;
use openai::pipelines::{
    llama::{LlamaLoader, LlamaSpecificConfig},
    mistral::{Mistral7BLoader, Mistral7BSpecificConfig},
    ModelLoader,
};

#[derive(Debug, Subcommand)]
pub enum ModelSelected {
    /// Select the llama7b model.
    Llama7b {
        #[arg(long)]
        repeat_last_n: usize,
    },

    /// Select the llama13b model.
    Llama13b {
        #[arg(long)]
        repeat_last_n: usize,
    },

    /// Select the llama70b model.
    Llama70b {
        #[arg(long)]
        repeat_last_n: usize,
    },

    /// Select the mistral7b model.
    Mistral7b {
        #[arg(long)]
        repeat_penalty: f32,
        #[arg(long)]
        repeat_last_n: usize,
        #[arg(long)]
        use_flash_attn: bool,
    },
}

impl ToString for ModelSelected {
    fn to_string(&self) -> String {
        match self {
            ModelSelected::Llama7b { repeat_last_n: _ } => "llama7b".to_string(),
            ModelSelected::Llama13b { repeat_last_n: _ } => "llama13b".to_string(),
            ModelSelected::Llama70b { repeat_last_n: _ } => "llama70b".to_string(),
            ModelSelected::Mistral7b {
                repeat_penalty: _,
                repeat_last_n: _,
                use_flash_attn: _,
            } => "mistral7b".to_string(),
        }
    }
}

pub fn get_model_loader<'a>(selected_model: ModelSelected) -> (Box<dyn ModelLoader<'a>>, String) {
    match selected_model {
        ModelSelected::Llama7b { repeat_last_n } => (
            Box::new(LlamaLoader::new(
                LlamaSpecificConfig::new(repeat_last_n),
                "llama7b".to_string(),
            )),
            "meta-llama/Llama-27b-chat-hf".to_string(),
        ),
        ModelSelected::Llama13b { repeat_last_n } => (
            Box::new(LlamaLoader::new(
                LlamaSpecificConfig::new(repeat_last_n),
                "llama13b".to_string(),
            )),
            "meta-llama/Llama-213b-chat-hf".to_string(),
        ),
        ModelSelected::Llama70b { repeat_last_n } => (
            Box::new(LlamaLoader::new(
                LlamaSpecificConfig::new(repeat_last_n),
                "llama70b".to_string(),
            )),
            "meta-llama/Llama-270b-chat-hf".to_string(),
        ),
        ModelSelected::Mistral7b {
            repeat_penalty,
            repeat_last_n,
            use_flash_attn,
        } => (
            Box::new(Mistral7BLoader::new(Mistral7BSpecificConfig::new(
                repeat_penalty,
                repeat_last_n,
                use_flash_attn,
            ))),
            "alpindale/mistral-7b-safetensors".to_string(),
        ),
    }
}

pub mod openai;
pub mod paged_attention;
