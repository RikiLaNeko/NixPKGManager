use colored::*;
use inquire::Text;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use regex::Regex;



const ASCII_ART: &str = r#"
 _       _________          _______  _        _______  _______  _______  _        _______  _______  _______  _______
( (    /|\__   __/|\     /|(  ____ )| \    /\(  ____ \(       )(  ___  )( (    /|(  ___  )(  ____ \(  ____ \(  ____ )
|  \  ( |   ) (   ( \   / )| (    )||  \  / /| (    \/| () () || (   ) ||  \  ( || (   ) || (    \/| (    \/| (    )|
|   \ | |   | |    \ (_) / | (____)||  (_/ / | |      | || || || (___) ||   \ | || (___) || |      | (__    | (____)|
| (\ \) |   | |     ) _ (  |  _____)|   _ (  | | ____ | |(_)| ||  ___  || (\ \) ||  ___  || | ____ |  __)   |     __)
| | \   |   | |    / ( ) \ | (      |  ( \ \ | | \_  )| |   | || (   ) || | \   || (   ) || | \_  )| (      | (\ (
| )  \  |___) (___( /   \ )| )      |  /  \ \| (___) || )   ( || )   ( || )  \  || )   ( || (___) || (____/\| ) \ \__
|/    )_)\_______/|/     \||/       |_/    \/(_______)|/     \||/     \||/    )_)|/     \|(_______)(_______/|/   \__/

NixPKGManager - Simplify your Nix package management
"#;

fn main() {
    println!("{}", ASCII_ART.cyan());

    let mut aliases = HashMap::new();
    // Définir les alias
    aliases.insert("i", "install");
    aliases.insert("rm", "remove");
    aliases.insert("ls", "list");
    aliases.insert("up", "update");
    aliases.insert("s", "search");
    aliases.insert("cfg", "config");
    aliases.insert("e", "edit");
    aliases.insert("h", "help");

    loop {
        let input = Text::new("nix-manager> ")
            .prompt()
            .unwrap_or_else(|_| "exit".to_string()); // Ctrl+D -> quitte proprement

        if input.trim() == "exit" {
            println!("{}", "👋 Bye!".green());
            break;
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        // Vérifie si l'entrée est un alias
        let command = aliases.get(args[0]).unwrap_or(&args[0]);

        match *command {
            "install" => {
                if args.len() > 1 {
                    install(args[1]);
                } else {
                    println!(
                        "{}",
                        "Erreur: Aucune cible spécifiée pour l'installation".red()
                    );
                }
            }
            "remove" => {
                if args.len() > 1 {
                    remove(args[1]);
                } else {
                    println!(
                        "{}",
                        "Erreur: Aucune cible spécifiée pour la suppression".red()
                    );
                }
            }
            "list" => list(),
            "update" => update(),
            "search" => {
                if args.len() > 1 {
                    search(args[1]);
                } else {
                    println!(
                        "{}",
                        "Erreur: Aucun mot-clé spécifié pour la recherche".red()
                    );
                }
            }
            "config" => {
                if args.len() > 2 {
                    config(args[1], args[2]);
                } else {
                    println!(
                        "{}",
                        "Erreur: Action ou paquet manquant pour la configuration".red()
                    );
                }
            }
            "edit" => {
                if args.len() > 1 {
                    config("edit", args[1]);
                } else {
                    println!("{}", "Erreur: Nom de paquet manquant pour l'édition".red());
                }
            }
            "help" => {
                println!(
                    "{}",
                    "📚 Commandes disponibles:
                    - install [package]: Installe un paquet
                    - remove [package]: Supprime un paquet
                    - list: Liste les paquets installés
                    - update: Met à jour les paquets
                    - search [keyword]: Recherche des paquets
                    - config add [package]: Ajoute un paquet à la configuration
                    - config remove [package]: Supprime un paquet de la configuration
                    - config edit: Ouvre le fichier de configuration
                    - help: Affiche ce message d'aide
                    - exit: Quitte le programme"
                        .blue()
                );
            }
            _ => println!("{}", "Commande inconnue! Tapez 'exit' pour quitter.".red()),
        }
    }
}

// === Commandes CLI ===

fn install(package: &str) {
    println!("{} {}...", "➡ Installation de".yellow(), package);
    let status = Command::new("nix")
        .args(["profile", "install", &format!("nixpkgs#{}", package)])
        .status()
        .expect("Erreur lors de l'installation");
    if status.success() {
        println!("{} {} installé avec succès!", "✔".green(), package);
    } else {
        eprintln!("{} Échec de l'installation de {}", "✖".red(), package);
    }
}

fn remove(package: &str) {
    println!("{} {}...", "➡ Suppression de".yellow(), package);
    let status = Command::new("nix")
        .args(["profile", "remove", &format!("{}",package)])
        .status()
        .expect("Erreur lors de la suppression");
    if status.success() {
        println!("{} {} supprimé avec succès!", "✔".green(), package);
    } else {
        eprintln!("{} Échec de la suppression de {}", "✖".red(), package);
    }
}

fn list() {
    println!("{} Liste des paquets installés:", "📦".blue());
    let output = Command::new("nix")
        .args(["profile", "list"])
        .output()
        .expect("Erreur lors de la récupération de la liste des paquets");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        eprintln!("{} Impossible de récupérer la liste des paquets", "✖".red());
    }
}

fn update() {
    println!("{} Mise à jour des paquets...", "🔄".cyan());
    let status = Command::new("nix")
        .args(["profile", "upgrade"])
        .status()
        .expect("Erreur lors de la mise à jour");
    if status.success() {
        println!("{} Mise à jour réussie!", "✔".green());
    } else {
        eprintln!("{} Échec de la mise à jour", "✖".red());
    }
}

fn search(keyword: &str) {
    println!("{} Recherche de paquets pour '{}':", "🔍".blue(), keyword);
    let output = Command::new("nix")
        .args(["search", "nixpkgs", keyword])
        .output()
        .expect("Erreur lors de la recherche");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        eprintln!("{} Aucun paquet trouvé", "✖".red());
    }
}
pub fn config(action: &str, package: &str) {
    // Vérifie si le programme est exécuté en root
    if unsafe { libc::geteuid() } != 0 {
        let args: Vec<String> = std::env::args().collect();
        let status = Command::new("sudo")
            .args(&args)
            .status()
            .expect("Impossible de relancer l'application en tant que superutilisateur");
        if !status.success() {
            eprintln!("\u{2716} Impossible de relancer l'application en tant que superutilisateur");
        }
        return;
    }

    let config_path = "/etc/nixos/configuration.nix";
    let content = fs::read_to_string(config_path).expect("Impossible de lire le fichier");
    let re = Regex::new(r"(?s)(environment\.systemPackages\s*=\s*with pkgs; \[)(.*?)(\];)").unwrap();

    if let Some(captures) = re.captures(&content) {
        let packages_block = captures.get(2).unwrap().as_str().trim();
        let mut packages: Vec<&str> = packages_block.lines().map(|s| s.trim()).collect();

        match action {
            "add" => {
                if packages.contains(&package) {
                    eprintln!("\u{2716} Le paquet '{}' est déjà présent", package);
                    return;
                }
                packages.push(package);
            }
            "remove" | "rm" => {
                if !packages.contains(&package) {
                    eprintln!("\u{2716} Le paquet '{}' n'est pas présent", package);
                    return;
                }
                packages.retain(|&p| p != package);
            }
            "edit" | "e" => {
                println!("Ouverture du fichier de configuration...");
                Command::new("nano").arg(config_path).status().unwrap();
                return;
            }
            _ => {
                eprintln!("\u{2716} Action inconnue");
                return;
            }
        }

        let new_packages_block = packages.join("\n    ");
        let new_content = re.replace(&content, format!("$1\n    {}\n$3", new_packages_block));

        fs::write(config_path, new_content.to_string()).expect("Impossible de modifier le fichier");
        println!("\u{2714} Modification appliquée!");

        Command::new("nixos-rebuild")
            .arg("switch")
            .status()
            .expect("Échec de la reconstruction du système");
    } else {
        eprintln!("\u{2716} Ligne 'environment.systemPackages' non trouvée");
    }
}
