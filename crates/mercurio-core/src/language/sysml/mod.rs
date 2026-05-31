pub mod parser;

pub use mercurio_sysml::SysmlLanguageModule;
pub use parser::{compile_text, compile_text_with_context, parse};
