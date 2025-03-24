use std::collections::HashMap;

use crate::error::CompilerError;

use super::Type;

pub struct Context<'a> {
    scope: HashMap<String, Type>,
    function_return: Option<Option<Type>>,
    parent: Option<&'a Context<'a>>
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Self {
            scope: HashMap::new(),
            function_return: None,
            parent: None,
        }
    }

    /// Creates a new "inner" context with this one as its parent. Used when
    /// entering a new, inner scope
    pub fn inner(&'a self) -> Self {
        Self {
            scope: HashMap::new(),
            function_return: None,
            parent: Some(self),
        }
    }

    pub fn add_name(&mut self, n: String, t: Type) -> Result<(), CompilerError> {
        // Only checking current scope (not parents) cause we can re-define a
        // parent binding, but not one in our own scope
        if self.scope.contains_key(&n) {
            return Err(CompilerError::SemanticError("Name defined twice within scope"));
        }

        self.scope.insert(n, t);

        Ok(())
    }

    pub fn get_type(&self, n: &str) -> Option<&Type> {
        match self.scope.get(n) {
            Some(t) => Some(t),
            None => match self.parent {
                Some(p) => p.get_type(n),
                None => None,
            }
        }
    }

    pub fn return_type(&self) -> Option<Option<&Type>> {
        match &self.function_return {
            Some(t) => Some(t.as_ref()),
            None => match self.parent {
                Some(p) => p.return_type(),
                None => None,
            }
        }
    }

    pub fn set_return_type(&mut self, t: Option<Type>) -> Result<(), CompilerError> {
        if self.return_type().is_some() {
            // If we're trying to redefine the return type, it means we're
            // (somehow) defining a function while within another
            return Err(CompilerError::SemanticError("Cannot nest functions"));
        }

        self.function_return = Some(t);

        Ok(())
    }
}
