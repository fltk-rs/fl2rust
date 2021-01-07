#![allow(dead_code)]

use crate::reserved;
use crate::utils;
use std::sync::atomic;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Global,
    Comment,
    Decl,
    Scope(bool, Option<Vec<String>>),
    Class,
    /// parent_name
    Function(Option<Vec<String>>),
    ///  widget_type, parent, is_parent, props
    Member(String, Option<Vec<String>>, bool, Vec<String>),
    /// props
    Property(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ident: String,
    pub typ: TokenType,
}

impl Token {
    pub fn new(ident: String, typ: TokenType) -> Self {
        Self { ident, typ }
    }
}

static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

pub fn parse(file: &str) -> Vec<Token> {
    let mut tok_vec: Vec<Token> = vec![];
    let mut parent: Vec<String> = vec![];
    let mut curr_widget: Option<String> = None;
    let mut last_scope = TokenType::Scope(false, None);
    let mut last = Token::new("".to_string(), TokenType::Global);
    let lines = file.lines();
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let words = utils::sanitize_words(words);
        let mut ast = Token::new("".to_string(), TokenType::Global);
        if let Some(first) = words.get(0) {
            assert!(
                reserved::is_fluid_reserved(&first)
                    || first.starts_with("Fl_")
                    || first.contains("MenuItem")
            );
            match first.as_str() {
                // comment
                "#" => ast.typ = TokenType::Comment,
                "decl" => ast.typ = TokenType::Decl,
                "}" => {
                    if let Some(w) = words.get(1) {
                        if w == "{" {
                            ast.typ = TokenType::Scope(true, Some(parent.clone()));
                            last_scope = ast.typ.clone();
                        }
                    } else {
                        parent.pop();
                        ast.typ = TokenType::Scope(false, Some(parent.clone()));
                    }
                }
                "class" => {
                    if words.len() > 2 {
                        ast.ident = words[1].to_string();
                        ast.typ = TokenType::Class;
                        parent.push(format!("class {}", ast.ident.clone()));
                    }
                }
                "Function" => {
                    if words.len() > 2 {
                        for i in 0..words.len() {
                            let name = utils::unbracket(&words[1]).to_string();
                            if words[i] == "return_type" {
                                ast.ident = format!(
                                    // "{} -> {}",
                                    "{} -> Self", // just support Self for the time being
                                    name,
                                    // utils::unbracket(&words[i + 1]).to_string()
                                );
                                ast.typ = TokenType::Function(Some(parent.clone()));
                                break;
                            } else {
                                ast.ident = name.clone();
                                ast.typ = TokenType::Function(Some(parent.clone()));
                            }
                        }
                        parent.push(format!("Function {}", ast.ident.clone()));
                    }
                }
                _ => {
                    if first.starts_with("Fl_") || *first == "MenuItem" {
                        let temp: String;
                        if !words[1].starts_with('{') {
                            temp = words[1].to_string();
                        } else {
                            let val = COUNTER.load(atomic::Ordering::Relaxed);
                            temp = format!("fl2rust_gen_widget_{}", val);
                            COUNTER.store(val + 1, atomic::Ordering::Relaxed);
                        }
                        curr_widget = Some(temp.clone());
                        parent.push(temp.clone());
                        ast.ident = temp.clone();
                        if let TokenType::Scope(true, p) = &last_scope {
                            if let Some(parent_vec) = p {
                                ast.typ = TokenType::Member(
                                    utils::de_fl(first),
                                    Some(parent_vec.clone()),
                                    false,
                                    vec![],
                                );
                            }
                        } else {
                            ast.typ = TokenType::Member(utils::de_fl(first), None, false, vec![]);
                        }          
                    } else if reserved::is_widget_prop(first) {
                        if let Some(curr) = curr_widget.clone() {
                            ast.ident = curr.clone();
                            ast.typ = TokenType::Property(words);
                        }
                    } else {
                        //
                    }
                }
            }
        }
        assert!(!reserved::is_rust_reserved(&ast.ident));
        if ast.typ == TokenType::Scope(false, None) && last.typ == TokenType::Scope(false, None) {
            parent.pop();
        }
        last = ast.clone();
        tok_vec.push(ast.clone());
        if let TokenType::Member(_, _, _, _) = ast.typ {
            tok_vec.push(Token {
                ident: String::from(""),
                typ: TokenType::Scope(true, Some(parent.clone())),
            })
        }
    }
    add_props(tok_vec)
}

pub fn add_props(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut tok_vec: Vec<Token> = vec![];
    for i in 0..tokens.len() {
        if let TokenType::Property(v) = &tokens[i].typ {
            if let Some(p) = tokens.get(i + 1) {
                if let TokenType::Property(v1) = &p.typ {
                    let mut v = v.clone();
                    v.append(&mut v1.clone());
                    tokens[i].typ = TokenType::Property(v.to_vec());
                }
            }
        }
    }
    for i in 0..tokens.len() {
        if let TokenType::Property(v) = &tokens[i].typ {
            let mut elem = Token::new("".to_string(), TokenType::Global);
            elem.ident = tokens[i - 2].ident.clone();
            if let TokenType::Member(parent_typ, parent, is_parent, _) = &tokens[i - 2].typ
            {
                elem.typ = TokenType::Member(
                    parent_typ.clone(),
                    parent.clone(),
                    *is_parent,
                    v.clone(),
                );
                tok_vec.push(elem);
            }
        } else {
            tok_vec.push(tokens[i].clone());
        }
    }
    let mut tok_vec2: Vec<Token> = vec![];
    let mut i = 0;
    while i < tok_vec.len() {
        if let TokenType::Member(t, v, _, props) = &tok_vec[i].typ {
            if !props.is_empty() {
                if let TokenType::Scope(true, _) = &tok_vec[i + 1].typ {
                    let old = tok_vec[i].ident.clone(); 
                    let tok = Token {
                        ident: old,
                        typ: TokenType::Member(t.clone(), v.clone(), true, props.clone()),
                    };
                    tok_vec2.push(tok);
                } else {
                    tok_vec2.push(tok_vec[i].clone());
                }
            } else {
                //
            }
        } else if let TokenType::Scope(_, vec) = &tok_vec[i].typ {
            if let Some(v) = vec {
                let len = v.len();
                if v.len() > 2 {
                    if v[len - 1] == v[len - 2] {
                        //
                    } else {
                        tok_vec2.push(tok_vec[i].clone());
                    } 
                } else {
                    tok_vec2.push(tok_vec[i].clone());
                }
            } else {
                tok_vec2.push(tok_vec[i].clone());
            }
        } else {
            tok_vec2.push(tok_vec[i].clone());
        }
        i += 1;
    }
    tok_vec2
}
