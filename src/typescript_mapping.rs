pub fn graphql_to_typescript(type_name: &str) -> Option<String> {
    match type_name {
        "ID" => Some("number".to_string()),
        "ID!" => Some("number".to_string()),
        "[Int]" => Some("number[]".to_string()),
        "Int!" => Some("number".to_string()),
        "Int" => Some("number | null".to_string()),
        "[String]" => Some("string[]".to_string()),
        "String!" => Some("string".to_string()),
        "String" => Some("string | null".to_string()),
        "Boolean" => Some("boolean".to_string()),
        "Boolean!" => Some("boolean".to_string()),
        _ => {
            if type_name.ends_with('!') {
                // Si le type se termine par `!`, alors pas de `| null`
                let base_type = &type_name[..type_name.len() - 1];
                if base_type.starts_with('[') && base_type.ends_with(']') {
                    // Si le type commence et se termine par `[` et `]`, alors `[]`
                    Some(format!("{}[]", &base_type[1..base_type.len() - 1]))
                } else {
                    Some(base_type.to_string())
                }
            } else if type_name.starts_with('[') && type_name.ends_with(']') {
                // Si le type commence par `[`, se termine par `]` mais n'a pas de `!`, alors `[] | null`
                Some(format!("{}[] | null", &type_name[1..type_name.len() - 1]))
            } else {
                // Sinon, `| null`
                Some(format!("{} | null", type_name))
            }
        }
    }
}