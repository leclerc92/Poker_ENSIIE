use std::io::{self, Write};
use crate::card::Card;
use crate::player::Player;
use crate::board::Board;

// Définir les constantes pour les codes couleur ANSI
const CYAN: &str = "\u{001B}[0;36m";
const GREEN: &str = "\u{001B}[0;32m";
const RESET: &str = "\u{001B}[0m";

/// Efface l'écran du terminal
pub fn clear_screen() {
    // Utilisation de la commande de nettoyage appropriée selon le système d'exploitation
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .expect("échec de l'effacement de l'écran");
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("clear")
            .status()
            .expect("échec de l'effacement de l'écran");
    }
}

/// Met le programme en pause jusqu'à ce que l'utilisateur appuie sur Entrée
pub fn pause_program() {
    println!("\nAppuyez sur Entrée pour continuer... ou Ctrl-C à tout moment pour quitter le jeu");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Échec de la lecture de l'entrée");
}

/// Lit et valide une entrée utilisateur en fonction d'une plage de valeurs
pub fn get_valid_input(prompt: &str, min: i32, max: i32, error_message: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Échec du flush");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Échec de la lecture de l'entrée");

        match input.trim().parse::<i32>() {
            Ok(value) if value >= min && value <= max => return value,
            _ => println!("{}", error_message),
        }
    }
}

/// Renvoie la couleur associée à un joueur en fonction de son ID
pub fn get_player_color(player_id: usize) -> &'static str {
    match player_id {
        1 | 3 => CYAN,
        2 | 4 => GREEN,
        _ => RESET,
    }
}

/// Affiche l'état actuel du plateau de jeu
pub fn display_board(b: &Board) {
    pause_program();
    println!("-------------------------- Plateau de jeu --------------------------");

    // Nombre d'équipes
    let number_of_teams = b.get_number_of_teams();
    println!("Nombre d'équipes : {}", number_of_teams);

    // Couleurs pour les équipes
    let colors = [CYAN, GREEN];

    // Parcourir chaque équipe
    for team_id in 0..number_of_teams {
        // Appliquer une couleur différente pour chaque équipe
        print!("{}", colors[team_id % 2]);

        println!("=======================================================================================");
        println!("Équipe {} :", team_id + 1);
        println!("  - Score : {}", b.get_score_of_team(team_id));

        // Nombre de joueurs dans l'équipe
        let num_players = b.get_number_of_players_in_team(team_id);
        println!("  - Nombre de joueurs : {}", num_players);

        // Parcourir chaque joueur
        for player_index in 0..num_players {
            if let Some(p) = b.get_player(team_id, player_index) {
                println!("\n    Joueur ID {}:", p.get_player_id());

                // Afficher le pari de la joueuse
                println!("      Pari : {}", p.get_slate());

                // Cartes en main
                let hand_size = p.get_size_of_hand();
                println!("      Cartes en main ({}) :", hand_size);

                // Ligne supérieure
                for _ in 0..hand_size {
                    print!("  +---------+  ");
                }
                println!();

                // Ligne ID
                for card_index in 0..hand_size {
                    if let Some(c) = p.get_card_in_hand(card_index) {
                        print!("  | ID: {:2}  |  ", c.get_id());
                    }
                }
                println!();

                // Ligne Vide
                for _ in 0..hand_size {
                    print!("  |---------|  ");
                }
                println!();

                // Ligne Valeur
                for card_index in 0..hand_size {
                    if let Some(c) = p.get_card_in_hand(card_index) {
                        print!("  | Atk: {:2} |  ", c.get_value());
                    }
                }
                println!();

                // Ligne inférieure
                for _ in 0..hand_size {
                    print!("  +---------+  ");
                }
                println!();
            }
        }
        pause_program();
    }

    // Réinitialiser la couleur par défaut
    print!("{}", RESET);

    // Cartes mises de côté
    let out_of_game_cards = b.get_number_of_out_of_game_cards();
    println!("=======================================================================================");
    println!("\nCartes mises de côté ({}) :", out_of_game_cards);

    let cards_per_row = 5; // Nombre de cartes par ligne

    // Affichage des cartes en paquets de 5
    for start in (0..out_of_game_cards).step_by(cards_per_row) {
        let end = std::cmp::min(start + cards_per_row, out_of_game_cards);

        // Ligne supérieure
        for _ in start..end {
            print!("  +---------+  ");
        }
        println!();

        // Ligne ID
        for card_index in start..end {
            if let Some(c) = b.get_out_of_game_card(card_index) {
                print!("  | ID: {:2}  |  ", c.get_id());
            }
        }
        println!();

        // Ligne Vide
        for _ in start..end {
            print!("  |---------|  ");
        }
        println!();

        // Ligne Valeur
        for card_index in start..end {
            if let Some(c) = b.get_out_of_game_card(card_index) {
                print!("  | Atk: {:2} |  ", c.get_value());
            }
        }
        println!();

        // Ligne inférieure
        for _ in start..end {
            print!("  +---------+  ");
        }
        println!();
    }

    println!("\n\n---------------------------------------------------------------------------------------");
    pause_program();
}

/// Demande à un joueur de faire un pari (victoire ou défaite)
pub fn ask_gamble(player: &Player) -> i32 {
    let mut prompt = String::new();
    let color = get_player_color(player.get_player_id());

    // Construire le message avec la couleur
    prompt.push_str(&format!(
        "{}\n> Joueur {} <\nVeuillez faire un pari ('0 = Defaite' ou '1 = Victoire') :\n{}",
        color, player.get_player_id(), RESET
    ));

    // Appeler la fonction de validation
    get_valid_input(&prompt, 0, 1, "RESULTAT INCORRECT ! Ecrivez '0 = Defaite' ou '1 = Victoire'")
}

/// Demande à un joueur de faire une mise de jetons
pub fn ask_chips_bet(player: &Player) -> i32 {
    let mut prompt = String::new();
    let color = get_player_color(player.get_player_id());
    let player_chips = player.get_chips() as i32;

    // Construire le message avec la couleur
    prompt.push_str(&format!(
        "{}\n> Joueur {} <\nVeuillez faire une mise de jeton ('0 à {}') :\n{}",
        color, player.get_player_id(), player_chips, RESET
    ));

    // Appeler la fonction de validation
    get_valid_input(&prompt, 0, player_chips, "MISE INCORRECTE !!")
}

/// Demande à un joueur de choisir le nombre de cartes qu'il souhaite jouer (2 maximum)
pub fn ask_number_of_played_cards(player: &Player) -> i32 {
    let mut prompt = String::new();
    let color = get_player_color(player.get_player_id());

    prompt.push_str(&format!(
        "{}> Joueur {} <\nVeuillez choisir le nombre de cartes que vous souhaitez jouer (2 MAXIMUM PAR TOUR) ! :{}",
        color, player.get_player_id(), RESET
    ));

    let mut number_of_card_played = get_valid_input(&prompt, 1, 2,
                                                    "RESULTAT INCORRECT ! Veuillez entrer un nombre entre 1 et 2.");

    // Vérifie si le joueur a moins de 2 cartes en main
    while player.get_size_of_hand() < 2 && number_of_card_played == 2 {
        println!("\nRESULTAT INCORRECT ! Vous n'avez qu'une seule carte en main.");
        number_of_card_played = get_valid_input(&prompt, 1, 1,
                                                "RESULTAT INCORRECT ! Veuillez entrer votre dernière carte.");
    }

    println!("Joueur {}, vous avez décidé de jouer {} carte(s) !",
             player.get_player_id(), number_of_card_played);

    number_of_card_played
}

/// Demande à un joueur de choisir une carte à jouer
pub fn ask_card(player: &Player) -> Option<&Card> {
    let color = get_player_color(player.get_player_id());
    print!("{}", color);

    // Affiche les cartes disponibles
    for _ in 0..player.get_size_of_hand() {
        print!("  +---------+  ");
    }
    println!();

    // Ligne ID
    for i in 0..player.get_size_of_hand() {
        print!("  | ID: {:2}  |  ", i);
    }
    println!();

    // Ligne Vide
    for _ in 0..player.get_size_of_hand() {
        print!("  |---------|  ");
    }
    println!();

    // Ligne Valeur
    for i in 0..player.get_size_of_hand() {
        if let Some(card) = player.get_card_in_hand(i) {
            print!("  | Atk: {:2} |  ", card.get_value());
        }
    }
    println!();

    // Ligne inférieure
    for _ in 0..player.get_size_of_hand() {
        print!("  +---------+  ");
    }

    let prompt = format!("\nChoisissez une carte (indice entre 0 et {}) :",
                         player.get_size_of_hand() - 1);

    let id_card = get_valid_input(&prompt, 0, (player.get_size_of_hand() - 1) as i32,
                                  "RESULTAT INCORRECT ! Veuillez choisir un indice valide.") as usize;

    player.get_card_in_hand(id_card) // Retourne la carte correspondant à l'indice choisi
}

/// Affiche les résultats de fin de jeu, y compris les scores des équipes et des joueurs
pub fn display_end_game(b: &Board) {
    pause_program();
    println!("---------------------------- Fin du jeu ----------------------------\n");

    // Afficher les informations sur les équipes
    let num_teams = b.get_number_of_teams();
    println!("Nombre d'équipes : {}\n", num_teams);

    // Variables pour déterminer l'équipe gagnante
    let mut highest_score = -1;
    let mut winning_team = -1;
    let mut tie = false;

    // Couleurs pour les équipes
    let colors = [CYAN, GREEN];

    // Première passe pour déterminer le score le plus élevé
    for team_id in 0..num_teams {
        let team_score = b.get_score_of_team(team_id) as i32;
        if team_score > highest_score {
            highest_score = team_score;
            winning_team = team_id as i32;
            tie = false;
        } else if team_score == highest_score {
            tie = true;
        }
    }

    for team_id in 0..num_teams {
        // Appliquer une couleur différente pour chaque équipe
        print!("{}", colors[team_id % 2]);

        // Informations sur l'équipe
        let team_score = b.get_score_of_team(team_id);
        println!("Équipe {} :", team_id + 1);
        println!("  - Score : {}", team_score);

        // Afficher les joueuses de l'équipe
        let num_players = b.get_number_of_players_in_team(team_id);
        println!("  - Nombre de joueuses : {}", num_players);

        for player_index in 0..num_players {
            if let Some(p) = b.get_player(team_id, player_index) {
                let player_id = p.get_player_id();
                let player_slate = p.get_slate();
                println!("    Joueuse {} :", player_id);
                println!("      Pari : {}", player_slate);

                // Cartes dans la main
                let hand_size = p.get_size_of_hand();
                print!("      Cartes en main ({}) : ", hand_size);

                for card_index in 0..hand_size {
                    if let Some(c) = p.get_card_in_hand(card_index) {
                        print!("{} ", c.get_value());
                    }
                }
                println!();
            }
        }
        println!();
    }

    // Réinitialiser la couleur par défaut
    print!("{}", RESET);

    // Afficher le message de félicitations après avoir affiché toutes les équipes
    if tie {
        println!("\n======== MATCH NUL ! Plusieurs équipes ont obtenu le score de {} points ========\n",
                 highest_score);
    } else {
        println!("\n======== FÉLICITATIONS À L'ÉQUIPE {} QUI GAGNE AVEC {} POINTS ! ========\n",
                 winning_team + 1, highest_score);
    }

    println!("\n--------------------------------------------------------------------");
}

/// Affiche un message générique à l'utilisateur
pub fn display_message(message: &str) {
    println!("{}", message);
}