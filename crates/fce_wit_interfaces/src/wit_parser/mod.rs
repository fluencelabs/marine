mod custom;
mod errors;
mod extracter;
mod embedder;

pub use errors::WITParserError;

pub use embedder::EmbedderConfig;
pub use embedder::embed_text_wit;

pub use extracter::extract_fce_wit;
pub use extracter::extract_text_wit;
