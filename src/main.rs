
use std::io::{self, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};
fn main() {
    println!("Programme d'exécution automatique de commandes.");
    println!("Caroline Jaffré");
    // J'utilise print et pas println pour écrire sur une seule ligne, donc il faut flush() pour afficher immédiatement
    print!("Entrez un intervalle de mise à jour (en minutes) : ");
    io::stdout().flush().unwrap();

    // La valeur entrée par l'utilisateur
    let mut intervalle = String::new();

    // Lecture
    io::stdin()
        .read_line(&mut intervalle)
        .expect("Erreur lecture intervalle");

    println!("Entrez la commande à exécuter :");
    let mut commande = String::new();
    io::stdin()
        .read_line(&mut commande)
        .expect("Erreur lecture commande");

    // On vérifie qu'il s'agit bien d'un nombre entier
    let test = intervalle.trim().parse::<u64>();
    match test {
        Ok(ok) => update(ok, &commande),
        Err(e) => println!("Erreur : {} : {}", e, intervalle),
    }
}

fn update(intervalle: u64, commande: &String) {
    println!("Lancement du programme avec intervalle de {} minutes", intervalle);
    let intervalle_secondes = intervalle * 60;
    let duree = Duration::from_secs(intervalle_secondes);
    let mut prochain_update = Instant::now() + duree;
    loop {
        // On vérifie l'OS, puis on exécute la commandes
        if cfg!(target_os = "windows") {
            let message_erreur = "Erreur cmd avec la commande : ".to_owned() + &commande;
            Command::new("cmd")
            .arg("/C")
            .arg(commande)
            .spawn()
            .expect(&message_erreur);
        } else {
            let message_erreur = "Erreur cmd avec la commande : ".to_owned() + &commande;
            Command::new("sh")
            .arg("-c")
            .arg(commande)
            .spawn()
            .expect(&message_erreur);
        }

        sleep(prochain_update - Instant::now());
        prochain_update += duree;
    }
}

