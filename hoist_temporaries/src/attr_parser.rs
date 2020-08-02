use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::collections::BTreeSet;

pub fn parse(
    token_stream: TokenStream,
    names: &mut BTreeSet<String>,
) -> Result<TokenStream, String> {
    let mut state = IdentDetectionState::Ignore;
    let mut new = TokenStream::new();

    for token in token_stream {
        new.extend(state.advance(token, names)?);
    }
    new.extend(state.flush());
    Ok(new)
}

enum IdentDetectionState {
    Ignore,
    SawHash(TokenTree),
    SawAttr,
    SawLet(Vec<TokenTree>),
    SawMut(Vec<TokenTree>),
}

impl IdentDetectionState {
    fn advance(
        &mut self,
        token: TokenTree,
        names: &mut BTreeSet<String>,
    ) -> Result<Vec<TokenTree>, String> {
        match self {
            IdentDetectionState::Ignore => {
                if eq_punct(&token, '#') {
                    *self = Self::SawHash(token);
                    return Ok(vec![]);
                } else if let TokenTree::Group(group) = token {
                    return Ok(vec![TokenTree::Group(Group::new(
                        group.delimiter(),
                        parse(group.stream(), names)?,
                    ))]);
                }
                Ok(vec![token])
            }
            IdentDetectionState::SawHash(_) => {
                if let TokenTree::Group(group) = token {
                    let new_group = TokenTree::Group(Group::new(
                        group.delimiter(),
                        parse(group.stream(), names)?,
                    ));
                    if Self::is_hoist_attr(&new_group) {
                        *self = Self::SawAttr;
                        Ok(vec![])
                    } else {
                        let prev = std::mem::replace(self, Self::Ignore);
                        if let IdentDetectionState::SawHash(hash) = prev {
                            Ok(vec![hash, new_group])
                        } else {
                            unreachable!()
                        }
                    }
                } else {
                    let prev = std::mem::replace(self, Self::Ignore);
                    if let IdentDetectionState::SawHash(hash) = prev {
                        Ok(vec![hash, token])
                    } else {
                        unreachable!()
                    }
                }
            }
            IdentDetectionState::SawAttr => {
                if eq_ident(&token, "let") {
                    *self = IdentDetectionState::SawLet(vec![token]);
                    return Ok(vec![]);
                }
                Err(Self::non_let_token_err(&[token]))
            }
            IdentDetectionState::SawLet(tokens) => {
                if let TokenTree::Ident(ident) = &token {
                    if &ident.to_string() == "mut" {
                        let mut tokens: Vec<TokenTree> = tokens.drain(..).collect();
                        tokens.push(token);
                        *self = IdentDetectionState::SawMut(tokens);
                        return Ok(vec![]);
                    } else {
                        names.insert(ident.to_string());
                        if let IdentDetectionState::SawLet(mut tokens) =
                            std::mem::replace(self, IdentDetectionState::Ignore)
                        {
                            tokens.push(token);
                            return Ok(tokens);
                        } else {
                            unreachable!();
                        }
                    }
                }
                Err(Self::non_let_token_err(&tokens))
            }
            IdentDetectionState::SawMut(tokens) => {
                if let TokenTree::Ident(ident) = &token {
                    names.insert(ident.to_string());
                    let mut tokens: Vec<TokenTree> = tokens.drain(..).collect();
                    tokens.push(token);
                    *self = IdentDetectionState::Ignore;
                    return Ok(tokens);
                }
                Err(Self::non_let_token_err(&tokens))
            }
        }
    }

    fn is_hoist_attr(group: &TokenTree) -> bool {
        if let TokenTree::Group(group) = group {
            if let Delimiter::Bracket = group.delimiter() {
                let mut stream: Vec<TokenTree> = group.stream().into_iter().collect();
                if stream.len() != 4 {
                    return false;
                }
                let hoist = stream.pop().unwrap();
                let punct2 = stream.pop().unwrap();
                let punct1 = stream.pop().unwrap();
                let hoist_temporaries = stream.pop().unwrap();

                return eq_ident(&hoist_temporaries, "hoist_temporaries")
                    && eq_punct(&punct1, ':')
                    && eq_punct(&punct2, ':')
                    && eq_ident(&hoist, "hoist");
            }
        }
        false
    }

    fn flush(self) -> Vec<TokenTree> {
        match self {
            IdentDetectionState::SawHash(tree) => vec![tree],
            IdentDetectionState::SawLet(vec) => vec,
            IdentDetectionState::SawMut(vec) => vec,
            IdentDetectionState::Ignore => vec![],
            IdentDetectionState::SawAttr => vec![],
        }
    }

    fn non_let_token_err(tokens: &[TokenTree]) -> String {
        format!("After #[hoist_temporaries::hoist] attribute, expected a let statement but instead got `{}`", tokens.iter().map(|t| format!("{}", t)).collect::<Vec<_>>().join(""))
    }
}

fn eq_ident(token: &TokenTree, value: &str) -> bool {
    if let TokenTree::Ident(ident) = token {
        return &ident.to_string() == value;
    }
    false
}

fn eq_punct(token: &TokenTree, value: char) -> bool {
    if let TokenTree::Punct(punct) = token {
        return punct.as_char() == value;
    }
    false
}
