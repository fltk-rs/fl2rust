use crate::parser;

pub fn unbracket(word: &str) -> &str {
    if word.starts_with('{') && word.ends_with('}') {
        &word[1..word.len() - 1]
    } else {
        word
    }
}

pub fn de_fl(word: &str) -> String {
    let s: String;
    if let Some(stripped) = word.strip_prefix("Fl_") {
        s = String::from(stripped).replace("_", "");
    } else {
        s = String::from(word);
    }
    s
}

pub fn sanitize_words(words: Vec<&str>) -> Vec<String> {
    let mut v = vec![];
    let mut s = "".to_string();
    let mut i = 0;
    while i < words.len() {
        if words[i].starts_with('{') && !words[i].ends_with('}') {
            s.push_str(words[i]);
            for j in i..words.len() {
                if words[j].ends_with('}') {
                    for elem in words.iter().take(j + 1).skip(i + 1) {
                        s.push(' ');
                        s.push_str(elem);
                        i += 1;
                    }
                }
            }
        } else {
            s.push_str(words[i]);
        }
        v.push(s.clone());
        s.clear();
        i += 1;
    }
    v
}

pub fn add_props(mut tokens: Vec<parser::Token>) -> Vec<parser::Token> {
    let mut tok_vec: Vec<parser::Token> = vec![];
    for i in 0..tokens.len() {
        if let parser::TokenType::Property(v) = &tokens[i].typ {
            if let Some(p) = tokens.get(i + 1) {
                if let parser::TokenType::Property(v1) = &p.typ {
                    let mut v = v.clone();
                    v.append(&mut v1.clone());
                    tokens[i].typ = parser::TokenType::Property(v.to_vec());
                }
            }
        }
    }
    for i in 0..tokens.len() {
        if let parser::TokenType::Property(v) = &tokens[i].typ {
            let mut elem = parser::Token::new("".to_string(), parser::TokenType::Global);
            elem.ident = tokens[i - 2].ident.clone();
            if let parser::TokenType::Member(parent_typ, parent, is_parent, _) = &tokens[i - 2].typ
            {
                elem.typ = parser::TokenType::Member(
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
    let mut tok_vec2: Vec<parser::Token> = vec![];
    let mut i = 0;
    while i < tok_vec.len() {
        if let parser::TokenType::Member(_, _, _, v) = &tok_vec[i].typ {
            if !v.is_empty() {
               tok_vec2.push(tok_vec[i].clone()); 
            } else {
                //
            }
        } else if let parser::TokenType::Scope(_, vec) = &tok_vec[i].typ {
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

pub fn global_to_pascal(input: &str) -> String {
    let mut s = String::from(input);
    if input.contains("FL_WHEN_") {
        s = s.replace("FL_WHEN_", "");
    } else {
        s = s.replace("FL_", "");
    }
    s = s.replace("_LABEL", "");
    if s == "No" {
        s = "None".to_string();
    }
    s = s.to_ascii_lowercase();
    let mut v: Vec<char> = s.chars().collect();
    if !v.is_empty() {
        v[0] = v[0].to_ascii_uppercase();
    }
    for i in 0..v.len() - 1 {
        if v[i] == '_' {
            v[i + 1] = v[i + 1].to_ascii_uppercase();
        }
    }
    let s: String = v.into_iter().collect();
    let s = s.replace("_", "");
    s
}