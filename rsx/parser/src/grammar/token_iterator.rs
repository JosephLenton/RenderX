use crate::error::Error;
use ::lookahead::lookahead;
use ::lookahead::Lookahead;
use ::proc_macro2::Delimiter;
use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;
use ::std::fmt::Debug;
use ::std::iter::Iterator;

#[derive(Clone, Debug)]
pub struct TokenIterator<I: Iterator<Item = TokenTree> + Clone + Debug> {
    iter: Lookahead<I>,
}

impl<I: Iterator<Item = TokenTree> + Clone + Debug> TokenIterator<I> {
    pub fn new<IntoI>(stream: IntoI) -> Self
    where
        IntoI: IntoIterator<Item = TokenTree, IntoIter = I>,
    {
        let iterator = stream.into_iter();
        Self {
            iter: lookahead(iterator),
            // iter: lookahead(Box::new(stream.into_iter())),
        }
    }

    pub fn peek(&mut self) -> Option<&TokenTree> {
        self.iter.lookahead(0)
    }

    pub fn is_lookahead_puncts(&mut self, cs: &[char]) -> bool {
        for (i, c) in cs.iter().enumerate() {
            if !self.is_lookahead_punct(*c, i) {
                return false;
            }
        }

        true
    }

    pub fn lookahead(&mut self, index: usize) -> Option<&TokenTree> {
        self.iter.lookahead(index)
    }

    pub fn is_lookahead_punct(&mut self, c: char, index: usize) -> bool {
        if let Some(TokenTree::Punct(punct)) = self.lookahead(index) {
            return punct.as_char() == c;
        }

        false
    }

    pub fn is_lookahead_ident(&mut self, index: usize) -> bool {
        if let Some(TokenTree::Ident(_)) = self.lookahead(index) {
            return true;
        }

        false
    }

    pub fn is_next_ident(&mut self) -> bool {
        if let Some(TokenTree::Ident(_)) = self.peek() {
            return true;
        }

        false
    }

    pub fn chomp_ident_or(&mut self, alt: &str) -> Result<String, Error> {
        if let Some(TokenTree::Ident(_)) = self.peek() {
            self.chomp_ident()
        } else {
            Ok(alt.to_string())
        }
    }

    pub fn is_next_punct(&mut self, c: char) -> bool {
        self.is_lookahead_punct(c, 0)
    }

    pub fn is_next_literal(&mut self) -> bool {
        if let Some(TokenTree::Literal(_)) = self.peek() {
            return true;
        }

        false
    }

    /// Returns true if empty.
    pub fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }

    /// Moves forward one item.
    ///
    /// Panics if called when there is no next item.
    pub fn chomp(&mut self) -> Result<TokenTree, Error> {
        if self.is_empty() {
            return Err(Error::ChompOnEmptyNode);
        }

        Ok(self.iter.next().unwrap())
    }

    pub fn chomp_ident(&mut self) -> Result<String, Error> {
        if let Some(TokenTree::Ident(ident)) = self.peek() {
            let ident_string = ident.to_string();
            self.chomp()?;

            return Ok(ident_string);
        }

        Err(Error::UnexpectedToken)
    }

    pub fn chomp_literal(&mut self) -> Result<String, Error> {
        if let Some(TokenTree::Literal(literal)) = self.peek() {
            let mut literal_string = literal.to_string();
            if literal_string.starts_with('"') {
                literal_string = literal_string.as_str()[1..literal_string.len() - 1].to_string();
            }

            self.chomp()?;

            return Ok(literal_string);
        }

        Err(Error::UnexpectedToken)
    }

    pub fn chomp_punct(&mut self, c: char) -> Result<(), Error> {
        if self.is_next_punct(c) {
            self.chomp()?;
            Ok(())
        } else {
            Err(Error::UnexpectedToken)
        }
    }

    pub fn chomp_puncts(&mut self, cs: &[char]) -> Result<(), Error> {
        for c in cs {
            self.chomp_punct(*c)?;
        }

        Ok(())
    }

    pub fn is_brace_group(&mut self) -> bool {
        self.is_group(Delimiter::Brace)
    }

    pub fn is_group(&mut self, delimiter: Delimiter) -> bool {
        if let Some(TokenTree::Group(group)) = self.peek() {
            return group.delimiter() == delimiter;
        }

        false
    }

    pub fn chomp_brace_group(&mut self) -> Result<TokenStream, Error> {
        match self.chomp()? {
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Brace {
                    Ok(group.stream())
                } else {
                    Err(Error::UnexpectedToken)
                }
            }
            _ => Err(Error::UnexpectedToken),
        }
    }

    pub fn chomp_group(&mut self, delimiter: Delimiter) -> Result<String, Error> {
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

        Err(Error::UnexpectedToken)
    }
}

#[cfg(test)]
mod is_lookahead_puncts {
    use super::*;
    use ::quote::quote;

    #[test]
    fn it_should_return_true_if_puncts_ahead() {
        let tokens = quote! {
            + + = +
        };

        let mut input = TokenIterator::new(tokens);
        assert!(input.is_lookahead_puncts(&['+']));
        assert!(input.is_lookahead_puncts(&['+', '+']));
        assert!(input.is_lookahead_puncts(&['+', '+', '=']));
        assert!(input.is_lookahead_puncts(&['+', '+', '=', '+']));
    }

    #[test]
    fn it_should_return_false_if_lookahead_overflows() {
        let tokens = quote! {
            + + = +
        };

        let mut input = TokenIterator::new(tokens);
        assert_eq!(false, input.is_lookahead_puncts(&['+', '+', '=', '+', '+']));
    }
}
