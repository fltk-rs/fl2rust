use crate::reserved;
use crate::utils;
use std::sync::atomic;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Global,
    Comment,
    Decl,
    Scope(bool, Vec<String>),
    Class,
    Function,
    ///  widget_type, is_parent, props
    Member(String, bool, Vec<String>),
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

/// Parses fl files to generate a token stream
pub fn parse(file: &str) -> Vec<Token> {
    let file = utils::fix_long_props(file);
    let mut tok_vec: Vec<Token> = vec![];
    let mut parent: Vec<String> = vec![];
    let mut curr_widget: Option<String> = None;
    let lines = file.lines();
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let words = utils::sanitize_words(words);
        let mut ast = Token::new("".to_string(), TokenType::Global);
        if let Some(first) = words.get(0) {
            match first.as_str() {
                // comment
                "#" => ast.typ = TokenType::Comment,
                "decl" => {
                    ast.typ = TokenType::Decl;
                    parent.push(format!("decl {}", ast.ident.clone()));
                    ast.typ = TokenType::Scope(true, parent.clone());
                }
                "}" => {
                    if let Some(w) = words.get(1) {
                        if w == "{" {
                            ast.typ = TokenType::Scope(true, parent.clone());
                        } else if w == "{}" {
                            parent.pop();
                            ast.typ = TokenType::Scope(false, parent.clone());
                        }
                    } else {
                        let temp = parent.pop();
                        if let Some(p) = temp {
                            if !p.contains("decl") {
                                ast.typ = TokenType::Scope(false, parent.clone());
                            }
                        }
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
                                ast.typ = TokenType::Function;
                                break;
                            } else {
                                ast.ident = name.clone();
                                ast.typ = TokenType::Function;
                            }
                        }
                        parent.push(format!("Function {}", ast.ident.clone()));
                    }
                }
                _ => {
                    if first.starts_with("Fl_") || *first == "MenuItem" || *first == "Submenu" {
                        let temp: String;
                        if !words[1].starts_with('{') {
                            temp = words[1].to_string();
                        } else {
                            let val = COUNTER.load(atomic::Ordering::Relaxed);
                            temp = format!("fl2rust_widget_{}", val);
                            COUNTER.store(val + 1, atomic::Ordering::Relaxed);
                        }
                        curr_widget = Some(temp.clone());
                        parent.push(format!("{} {}", first.clone(), temp.clone()));
                        ast.ident = temp.clone();
                        ast.typ = TokenType::Member(utils::de_fl(first), false, vec![]);
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
        tok_vec.push(ast.clone());
        if let TokenType::Member(_, _, _) = ast.typ {
            tok_vec.push(Token {
                ident: String::from(""),
                typ: TokenType::Scope(true, parent.clone()),
            })
        }
    }
    add_props(tok_vec)
}

/// Adds properties to the widgets
pub fn add_props(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut tok_vec: Vec<Token> = vec![];
    // appends token lines
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
    // add props to Member token, remove property tokens
    for i in 0..tokens.len() {
        if let TokenType::Property(v) = &tokens[i].typ {
            let mut elem = Token::new("".to_string(), TokenType::Global);
            elem.ident = tokens[i - 2].ident.clone();
            let label = v.iter().position(|x| x == "label");
            if let TokenType::Member(parent_typ, is_parent, _) = &tokens[i - 2].typ {
                if parent_typ == "Submenu" {
                    elem.ident = v[label.unwrap() + 1].to_string();
                }
                elem.typ = TokenType::Member(parent_typ.clone(), *is_parent, v.clone());
                tok_vec.pop();
                tok_vec.push(elem);
            }
        } else {
            tok_vec.push(tokens[i].clone());
        }
    }
    let mut tok_vec2: Vec<Token> = vec![];
    let mut i = 0;
    while i < tok_vec.len() {
        if let TokenType::Member(t, _, props) = &tok_vec[i].typ {
            if !props.is_empty() {
                let old = tok_vec[i].ident.clone();
                if let TokenType::Scope(true, _) = &tok_vec[i + 1].typ {
                    let tok = Token {
                        ident: old,
                        typ: TokenType::Member(t.clone(), true, props.clone()),
                    };
                    tok_vec2.push(tok);
                } else {
                    tok_vec2.push(tok_vec[i].clone());
                }
            }
        } else if let TokenType::Scope(true, vec) = &tok_vec[i].typ {
            let len = vec.len();
            if vec.len() > 2 {
                if vec[len - 1] == vec[len - 2] {
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
        i += 1;
    }
    tok_vec2
}
