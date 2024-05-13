use std::env;
use std::fs;
mod typescript_mapping;

fn main() {
    // Récupére les arguments de ligne de commande
    let args: Vec<String> = env::args().collect();

    // Vérifie que l'utilisateur a fourni un chemin de fichier
    if args.len() != 3 {
        eprintln!("Usage: {} chemin_vers_le_fichier.graphql nom_du_fichier.ts", args[0]);
        return;
    }

    // Récupére le chemin vers le fichier GraphQL et le nom du fichier de sortie à partir des arguments
    let input_file = &args[1];
    let output_file = &args[2];

    // Lire le contenu du fichier GraphQL
    let contents = match fs::read_to_string(input_file) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Erreur de lecture du fichier GraphQL : {}", err);
            return;
        }
    };

    // Extraire les types définis du fichier GraphQL
    let types = extract_types(&contents);

    // Générer le contenu du fichier TypeScript
    let ts_content = generate_typescript(&types);

    // Écrire le contenu dans le fichier TypeScript de sortie
    match fs::write(output_file, ts_content) {
        Ok(_) => println!("Fichier TypeScript généré avec succès : {}", output_file),
        Err(err) => eprintln!("Erreur lors de l'écriture du fichier TypeScript : {}", err),
    }
}

// Fonction pour extraire les types définis du fichier GraphQL
fn extract_types(contents: &str) -> Vec<(String, Vec<(String, String)>)> {
    let mut types = Vec::new();
    let mut current_type = String::new();
    let mut current_fields = Vec::new();

    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with("type") {
            if !current_type.is_empty() {
                types.push((current_type.clone(), current_fields.clone()));
                current_fields.clear();
            }
            current_type = line
                .split_whitespace()
                .nth(1)
                .unwrap_or_default()
                .trim_end_matches('{')
                .to_string();
        } else if line.starts_with("}") {
            if !current_type.is_empty() {
                types.push((current_type.clone(), current_fields.clone()));
                current_type.clear();
                current_fields.clear();
            }
        } else if !line.is_empty() && !line.starts_with("#") && line.contains(":") {
            let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
            let field_name = parts[0].to_string();
            let field_type = parts[1].to_string();
            current_fields.push((field_name, field_type));
        }
    }

    types
}

// Fonction pour générer le contenu du fichier TypeScript
fn generate_typescript(types: &[(String, Vec<(String, String)>)]) -> String {
    let mut ts_content = String::new();

    for (type_name, fields) in types {
        ts_content.push_str(&format!("export type {} = {{\n", type_name));
        for (field_name, field_type) in fields {
            let ts_type = match typescript_mapping::graphql_to_typescript(field_type) {
                Some(ts_type) => ts_type,
                None => continue,
            };
            ts_content.push_str(&format!("    {}: {},\n", field_name, ts_type));
        }
        ts_content.push_str("};\n\n");
    }

    ts_content
}