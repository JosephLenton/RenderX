use crate::ASTError;
use ::proc_macro2::token_stream::IntoIter;
use ::proc_macro2::Ident;
use ::proc_macro2::Punct;
use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;
use ::std::iter::Iterator;

#[derive(Clone, Debug)]
pub struct TokenIterator {
    iter: IntoIter,
    next: Option<TokenTree>,
}

impl TokenIterator {
    pub fn new(stream: TokenStream) -> Self {
        let mut iter = stream.into_iter();
        let next = iter.next();

        Self { iter, next }
    }

    /// Returns the next token, if there is one.
    /// It returns None if there are no more tokens.
    pub fn peek(&self) -> Result<&TokenTree, ASTError> {
        self.maybe_peek().ok_or(ASTError::PeekOnEmptyNonde)
    }

    pub fn maybe_peek(&self) -> Option<&TokenTree> {
        self.next.as_ref()
    }

    pub fn is_next_ident(&self) -> bool {
        if let Some(TokenTree::Ident(_)) = self.maybe_peek() {
            return true;
        }

        false
    }

    pub fn chomp_ident_or(&mut self, alt: &str) -> Result<String, ASTError> {
        if let Some(TokenTree::Ident(ident)) = &self.next {
            self.chomp_ident()
        } else {
            Ok(alt.to_string())
        }
    }

    pub fn is_next_punct(&self, other: &Punct) -> bool {
        if let Some(TokenTree::Punct(punct)) = self.maybe_peek() {
            return punct.as_char() == other.as_char() && punct.spacing() == other.spacing();
        }

        false
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.next.is_none()
    }

    /// Moves forward one item.
    ///
    /// Note that when this is empty, you can still continue
    /// to chomp on tokens. Forever.
    pub fn chomp(&mut self) -> Result<(), ASTError> {
        if self.next.is_none() {
            return Err(ASTError::ChompOnEmptyNonde);
        }

        self.next = self.iter.next();
        Ok(())
    }

    pub fn chomp_ident(&mut self) -> Result<String, ASTError> {
        if let Some(TokenTree::Ident(ident)) = &self.next {
            let ident_string = ident.to_string();
            self.chomp()?;

            return Ok(ident_string);
        }

        Err(ASTError::UnexpectedToken)
    }

    pub fn chomp_punct(&mut self, other: &Punct) -> Result<(), ASTError> {
        if self.is_next_punct(other) {
            self.chomp()?;
            Ok(())
        } else {
            Err(ASTError::UnexpectedToken)
        }
    }
}
