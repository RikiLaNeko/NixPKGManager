# NixPKGManager

NixPKGManager est un gestionnaire de paquets écrit en Rust. Ce projet vise à fournir une solution légère et rapide pour la gestion des paquets sur des systèmes basés sur Nix.

## Fonctionnalités

- Installation de paquets
- Mise à jour de paquets
- Suppression de paquets
- Recherche de paquets

## Prérequis

- Rust (version 1.50 ou supérieure)
- Cargo (gestionnaire de paquets pour Rust)

## Installation

Clonez le dépôt et compilez le projet :

```bash
git clone https://github.com/RikiLaNeko/NixPKGManager.git
cd NixPKGManager
cargo build --release
```

## Utilisation

Voici quelques exemples de commandes que vous pouvez utiliser avec NixPKGManager :
Installer un paquet
```bash

./nixpkgmanager install <nom_du_paquet>
```
Mettre à jour un paquet
```bash

./nixpkgmanager update <nom_du_paquet>
```
Supprimer un paquet
```bash

./nixpkgmanager remove <nom_du_paquet>
```
Rechercher un paquet
```bash

./nixpkgmanager search <nom_du_paquet>
```
## Contribuer

Les contributions sont les bienvenues ! Pour commencer, veuillez consulter les problèmes ouverts et soumettez une pull request avec vos modifications.

    Forkez le projet
    Créez une branche de fonctionnalité (git checkout -b feature/AmazingFeature)
    Commitez vos modifications (git commit -m 'Add some AmazingFeature')
    Poussez vers la branche (git push origin feature/AmazingFeature)
    Ouvrez une pull request

## Licence

Distribué sous la licence MIT. Voir LICENSE pour plus d'informations.
## Auteurs

    RikiLaNeko - Développeur principal - RikiLaNeko
