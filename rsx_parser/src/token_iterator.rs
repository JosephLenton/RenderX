use crate::ASTError;
use ::proc_macro2::token_stream::IntoIter;
use ::proc_macro2::Delimiter;
use ::proc_macro2::Ident;
use ::proc_macro2::Punct;
use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;
use ::std::iter::Iterator;
use ::std::mem::swap;

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
    /// Panics if called when there is no next item.
    pub fn chomp(&mut self) -> Result<TokenTree, ASTError> {
        if self.next.is_none() {
            return Err(ASTError::ChompOnEmptyNonde);
        }

        let mut last = self.iter.next();
        swap(&mut self.next, &mut last);

        Ok(last.unwrap())
    }

    pub fn chomp_ident(&mut self) -> Result<String, ASTError> {
        if let Some(TokenTree::Ident(ident)) = &self.next {
            let ident_string = ident.to_string();
            self.chomp()?;

            return Ok(ident_string);
        }

        Err(ASTError::UnexpectedToken)
    }

    pub fn chomp_literal(&mut self) -> Result<String, ASTError> {
        if let Some(TokenTree::Literal(literal)) = &self.next {
            let mut literal_string = literal.to_string();
            if literal_string.starts_with('"') {
                literal_string = literal_string.as_str()[1..literal_string.len() - 1].to_string();
            }

            self.chomp()?;

            return Ok(literal_string);
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

    pub fn is_brace_group(&mut self) -> bool {
        self.is_group(Delimiter::Brace)
    }

    pub fn is_group(&mut self, delimiter: Delimiter) -> bool {
        if let Some(TokenTree::Group(group)) = &self.next {
            return group.delimiter() == delimiter;
        }

        false
    }

    pub fn chomp_brace_group(&mut self) -> Result<String, ASTError> {
        self.chomp_group(Delimiter::Brace)
    }

    pub fn chomp_group(&mut self, delimiter: Delimiter) -> Result<String, ASTError> {
        if let TokenTree::Group(group) = self.chomp()? {
            if group.delimiter() == delimiter {
                let mut group_string = group.to_string();
                if group_string.starts_with('{') {
                    group_string = group_string.as_str()[1..group_string.len() - 1]
                        .trim()
                        .to_string();
                }

                return Ok(group_string);
            }
        }

        Err(ASTError::UnexpectedToken)
    }
}
