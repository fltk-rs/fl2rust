pub const FLUID_RESERVED: &[&str] = &[
    "header_name",
    "code_name",
    "comment",
    "snap",
    "version",
    "gridx",
    "gridy",
    "Function",
    "return_type",
    "const",
    "do_not_include_H_from_C",
    "callback",
    "user_data",
    "class",
    "xywh",
    "box",
    "code",
    "code0",
    "code1",
    "code2",
    "code3",
    "code4",
    "when",
    "labeltype",
    "labelsize",
    "labelfont",
    "labelcolor",
    "label",
    "selection_color",
    "textfont",
    "tooltip",
    "color",
    "minimum",
    "maximum",
    "step",
    "type",
    "align",
    "open",
    "selected",
    "value",
    "decl",
    "down_box",
    "shortcut",
    "hotspot",
    "image",
    "hide",
    "modal",
    "private",
    "global",
    "}",
    "#",
];

pub const WIDGET_PROPS: &[&str] = &[
    "xywh",
    "box",
    "when",
    "labeltype",
    "labelsize",
    "labelfont",
    "labelcolor",
    "label",
    "selection_color",
    "textfont",
    "tooltip",
    "color",
    "minimum",
    "maximum",
    "step",
    "type",
    "align",
    "selected",
    "value",
    "down_box",
    "shortcut",
    "hotspot",
    "image",
    "hide",
    "visible",
    "modal",
];

pub const RUST_RESERVED: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof",
    "unsized", "virtual", "yield", "try", "union", "dyn", "bool", "char", "i8", "i16", "i32",
    "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64", "array",
    "slice", "str", "tuple",
];

pub fn is_fluid_reserved(ident: &str) -> bool {
    FLUID_RESERVED.contains(&ident)
}

pub fn is_rust_reserved(ident: &str) -> bool {
    RUST_RESERVED.contains(&ident)
}

pub fn is_widget_prop(token: &str) -> bool {
    WIDGET_PROPS.contains(&token)
}
