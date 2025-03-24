mod context;
pub mod translation_unit;
pub mod types;
pub mod declaration;
pub mod statements;
pub mod expressions;

pub use translation_unit::TranslationUnit;
pub use types::Type;
pub use declaration::Declaration;
pub use expressions::Expression;
pub use statements::Statement;
pub use context::Context;

use crate::error::CompilerError;

pub trait SemanticUnit {
    /// Verifies that the given object (a declaration, expr, or statement) is
    /// semantically valid (e.g. types match, etc)
    fn verify(&self) -> Result<(), CompilerError> {
        let mut context = Context::new();
        self.verify_with_context(&mut context)
    }

    fn verify_with_context(&self, context: &mut Context) -> 
        Result<(), CompilerError>;
}
