use crate::ASTError;
use ::lookahead::lookahead;
use ::lookahead::Lookahead;
use ::proc_macro2::Delimiter;
use ::proc_macro2::Ident;
use ::proc_macro2::Punct;
use ::proc_macro2::Spacing;
use ::proc_macro2::TokenStream;
use ::proc_macro2::TokenTree;
use ::std::iter::Iterator;
use ::std::mem::replace;
use ::std::vec::IntoIter;

#[derive(Clone, Debug)]
pub struct TokenIterator {
    iter: Lookahead<IntoIter<TokenTree>>,
}

impl TokenIterator {
    pub fn new(stream: TokenStream) -> Self {
        let new_stream = flatten(stream);

        Self {
            iter: lookahead(new_stream.into_iter()),
        }
    }

    /// Returns the next token, if there is one.
    /// It returns None if there are no more tokens.
    pub fn peek_or_error(&mut self) -> Result<&TokenTree, ASTError> {
        self.peek().ok_or(ASTError::PeekOnEmptyNode)
    }

    pub fn peek(&mut self) -> Option<&TokenTree> {
        self.iter.lookahead(0)
    }

    pub fn lookahead(&mut self, index: usize) -> Option<&TokenTree> {
        self.iter.lookahead(index)
    }

    pub fn lookahead_punct(&mut self, c: char, index: usize) -> bool {
        if let Some(TokenTree::Punct(punct)) = self.lookahead(index) {
            return punct.as_char() == c;
        }

        false
    }

    pub fn is_next_ident(&mut self) -> bool {
        if let Some(TokenTree::Ident(_)) = self.peek() {
            return true;
        }

        false
    }

    pub fn chomp_ident_or(&mut self, alt: &str) -> Result<String, ASTError> {
        if let Some(TokenTree::Ident(_)) = self.peek() {
            self.chomp_ident()
        } else {
            Ok(alt.to_string())
        }
    }

    pub fn is_next_punct(&mut self, c: char) -> bool {
        self.lookahead_punct(c, 0)
    }

    /// Returns true if empty.
    pub fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }

    /// Moves forward one item.
    ///
    /// Panics if called when there is no next item.
    pub fn chomp(&mut self) -> Result<TokenTree, ASTError> {
        if self.is_empty() {
            return Err(ASTError::ChompOnEmptyNode);
        }

        Ok(self.iter.next().unwrap())
    }

    pub fn chomp_ident(&mut self) -> Result<String, ASTError> {
        if let Some(TokenTree::Ident(ident)) = self.peek() {
            let ident_string = ident.to_string();
            self.chomp()?;

            return Ok(ident_string);
        }

        Err(ASTError::UnexpectedToken)
    }

    pub fn chomp_literal(&mut self) -> Result<String, ASTError> {
        if let Some(TokenTree::Literal(literal)) = self.peek() {
            let mut literal_string = literal.to_string();
            if literal_string.starts_with('"') {
                literal_string = literal_string.as_str()[1..literal_string.len() - 1].to_string();
            }

            self.chomp()?;

            return Ok(literal_string);
        }

        Err(ASTError::UnexpectedToken)
    }

    pub fn chomp_punct(&mut self, c: char) -> Result<(), ASTError> {
        if self.is_next_punct(c) {
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
        if let Some(TokenTree::Group(group)) = self.peek() {
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

fn flatten(stream: TokenStream) -> Vec<TokenTree> {
    let mut new_stream = vec![];
    flatten_into(&mut new_stream, stream);
    new_stream
}

fn flatten_into(new_stream: &mut Vec<TokenTree>, stream: TokenStream) {
    for item in stream {
        match item {
            TokenTree::Group(group) => {
                let delimiter = group.delimiter();
                if delimiter != Delimiter::Brace {
                    let (opening_char, closing_char) = delimiter_chars(delimiter);

                    new_stream.push(TokenTree::Punct(Punct::new(opening_char, Spacing::Alone)));
                    flatten_into(new_stream, group.stream());
                    new_stream.push(TokenTree::Punct(Punct::new(closing_char, Spacing::Alone)));

                    continue;
                }

                new_stream.push(TokenTree::Group(group));
            }
            _ => new_stream.push(item),
        }
    }
}

fn delimiter_chars(delimiter: Delimiter) -> (char, char) {
    match delimiter {
        Delimiter::Bracket => ('[', ']'),
        Delimiter::Parenthesis => ('(', ')'),
        Delimiter::Brace => ('{', '}'),
        Delimiter::None => ('\0', '\0'),
    }
}
