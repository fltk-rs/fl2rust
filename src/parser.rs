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
                        }
                    } else {
                        ast.typ = TokenType::Scope(false, None);
                    }
                }
                "class" => {
                    if words.len() > 2 && line.contains("{open") {
                        ast.ident = words[1].to_string();
                        ast.typ = TokenType::Class;
                        parent.push(ast.ident.clone());
                    }
                }
                "Function" => {
                    if words.len() > 2 && line.contains("{open") {
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
                        parent.push(ast.ident.clone());
                    }
                }
                _ => {
                    if first.starts_with("Fl_") || *first == "MenuItem" {
                        let temp: String;
                        if !words[1].starts_with('{') {
                            temp = words[1].to_string();
                        } else {
                            let val = COUNTER.load(atomic::Ordering::Relaxed);
                            temp = format!("widget_{}", val);
                            COUNTER.store(val + 1, atomic::Ordering::Relaxed);
                        }
                        let mut is_parent = false;
                        if line.contains("{open") {
                            is_parent = true;
                            parent.push(temp.clone());
                        }
                        curr_widget = Some(temp.clone());
                        ast.ident = temp.clone();
                        if !parent.is_empty() && !parent[parent.len() - 1].contains('(') {
                            ast.typ = TokenType::Member(
                                utils::de_fl(first),
                                Some(parent.clone()),
                                is_parent,
                                vec![],
                            );
                        } else {
                            ast.typ =
                                TokenType::Member(utils::de_fl(first), None, is_parent, vec![]);
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
    utils::add_props(tok_vec)
}
