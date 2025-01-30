use clap::{Parser, Subcommand};
use std::process::Command;
use colored::*;
use inquire::Select;
use std::fs;

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

#[derive(Parser)]
#[command(name = "nix-manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a package
    Install {
        package: String,
    },
    /// Uninstall a package
    Remove {
        package: String,
    },
    /// List all installed packages
    List,
    /// Update all packages
    Update,
    /// Search for a package
    Search {
        keyword: String,
    },
    /// Edit the NixOS configuration
    Config {
        action: String,
        package: String,
    },
}

fn main() {
    println!("{}", ASCII_ART.cyan());
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { package } => {
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
        Commands::Remove { package } => {
            println!("{} {}...", "➡ Suppression de".yellow(), package);
            let status = Command::new("nix")
                .args(["profile", "remove", &format!("nixpkgs#{}", package)])
                .status()
                .expect("Erreur lors de la suppression");
            if status.success() {
                println!("{} {} supprimé avec succès!", "✔".green(), package);
            } else {
                eprintln!("{} Échec de la suppression de {}", "✖".red(), package);
            }
        }
        Commands::List => {
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
        Commands::Update => {
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
        Commands::Search { keyword } => {
            println!("{} Recherche de paquets pour '{}':", "🔍".blue(), keyword);
            let output = Command::new("nix")
                .args(["search", "nixpkgs", keyword])
                .output()
                .expect("Erreur lors de la recherche");
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let packages: Vec<&str> = stdout.lines().take(10).collect();
                if let Ok(choice) = Select::new("Sélectionne un paquet à installer:", packages).prompt() {
                    println!("Installation de {}...", choice);
                    let _ = Command::new("nix")
                        .args(["profile", "install", &format!("nixpkgs#{}", choice)])
                        .status();
                }
            } else {
                eprintln!("{} Aucun paquet trouvé", "✖".red());
            }
        }
        Commands::Config { action, package } => {
            let config_path = "/etc/nixos/configuration.nix";
            let content = fs::read_to_string(config_path).expect("Impossible de lire le fichier");
            let new_content = match action.as_str() {
                "add" => format!("{}
  environment.systemPackages = with pkgs; [ {} ];", content, package),
                "remove" => content.replace(&format!(" {}", package), ""),
                "edit" => {
                    println!("Ouvre le fichier de configuration...");
                    Command::new("nano").arg(config_path).status().unwrap();
                    return;
                }
                _ => {
                    eprintln!("{} Action inconnue", "✖".red());
                    return;
                }
            };
            fs::write(config_path, new_content).expect("Impossible de modifier le fichier");
            println!("{} Modification appliquée!", "✔".green());
            Command::new("nixos-rebuild").arg("switch").status().unwrap();
        }
    }
}

