pub fn de_fl(word: &str) -> String {
    let mut s: String;
    if let Some(stripped) = strip_prefix(word, "Fl_") {
        s = stripped.replace('_', "");
    } else {
        s = String::from(word);
    }
    if s == "Box" {
        s = "Frame".to_string();
    }
    s
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
    let s = s.replace('_', "");
    let ret = match s.as_str() {
        "Vert fill" => "VerticalFill",
        "Horz fill" => "HorizontalFill",
        "Vert knob" => "VerticalNice",
        "Horz knob" => "HorizontalNice",
        _ => s.as_str(),
    };
    ret.to_string()
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
