use crate::reserved;

pub fn unbracket(word: &str) -> &str {
    if word.starts_with('{') && word.ends_with('}') {
        &word[1..word.len() - 1]
    } else {
        word
    }
}

pub fn de_fl(word: &str) -> String {
    let mut s: String;
    if let Some(stripped) = strip_prefix(word, "Fl_") {
        s = stripped.replace("_", "");
    } else {
        s = String::from(word);
    }
    if s == "Box" {
        s = "Frame".to_string();
    }
    s
}

pub fn sanitize_words(words: Vec<&str>) -> Vec<String> {
    let mut v = vec![];
    let mut s = String::new();
    let mut i = 0;
    while i < words.len() {
        if words[i].starts_with('{') && !words[i].ends_with('}') {
            s.push_str(words[i]);
            i += 1;
            while i < words.len() && !words[i].ends_with('}') {
                s.push(' ');
                s.push_str(words[i]);
                i += 1;
            }
            if i < words.len() && words[i].ends_with('}') {
                s.push(' ');
                s.push_str(words[i]);
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
    let ret = match s.as_str() {
        "Vert fill" => "VerticalFill",
        "Horz fill" => "HorizontalFill",
        "Vert knob" => "VerticalNice",
        "Horz knob" => "HorizontalNice",
        _ => s.as_str(),
    };
    ret.to_string()
}

pub fn vec2menu(v: &[&str]) -> String {
    let mut s: String = String::new();
    for elem in v {
        s += unbracket(elem);
        s += "/";
    }
    s
}

pub fn strip_prefix(s: &str, pat: &str) -> Option<String> {
    match s.find(pat) {
        Some(idx) => {
            if idx == 0 {
                Some(s[pat.len()..].to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn fix_type(s: &str) -> &str {
    if s.contains("Slider") {
        return "Slider";
    }
    if s.contains("Browser") {
        return "Browser";
    }
    if s.contains("Button") {
        return "Button";
    }
    if s.contains("Input") {
        return "Input";
    }
    s
}

pub fn fix_long_props(s: &str) -> String {
    let mut temp = String::new();
    let lines: Vec<&str> = s.lines().collect();
    let mut i = 0;
    while i < lines.len() - 1 {
        temp.push_str(lines[i]);
        let words: Vec<&str> = lines[i + 1].split_whitespace().collect();
        if let Some(first) = words.get(0) {
            if reserved::is_fluid_reserved(&first)
                || first.starts_with("Fl_")
                || first.contains("MenuItem")
                || first.contains("Submenu")
            {
                temp.push('\n');
            } else {
                temp.push(' ');
            }
        }
        i += 1;
    }
    temp.push_str(lines[lines.len() - 1]);
    temp
}
