use clap::{Parser, Subcommand};
use std::process::Command;
use colored::*;
use inquire::Text;
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

fn main() {
    println!("{}", ASCII_ART.cyan());

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

        match args[0] {
            "install" | "i" if args.len() > 1 => install(args[1]),
            "remove" | "rm" if args.len() > 1 => remove(args[1]),
            "list" | "ls" => list(),
            "update" | "up" => update(),
            "search" | "s" if args.len() > 1 => search(args[1]),
            "config" | "cfg" if args.len() > 2 => config(args[1], args[2]),
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
        .args(["profile", "remove", &format!("nixpkgs#{}", package)])
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

fn config(action: &str, package: &str) {
    let config_path = "/etc/nixos/configuration.nix";
    let content = fs::read_to_string(config_path).expect("Impossible de lire le fichier");
    let new_content = match action {
        "add" => format!("{}
  environment.systemPackages = with pkgs; [ {} ];", content, package),
        "remove" | "rm" => content.replace(&format!(" {}", package), ""),
        "edit" | "e" => {
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

