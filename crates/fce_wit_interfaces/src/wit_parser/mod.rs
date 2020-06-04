mod custom;
mod errors;
mod extractor;
mod embedder;

pub use errors::WITParserError;

pub use embedder::EmbedderConfig;
pub use embedder::embed_text_wit;

pub use extractor::extract_fce_wit;
pub use extractor::extract_text_wit;
