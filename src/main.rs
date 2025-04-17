use std::ops::Deref;
use rand::Rng;
use crate::board::{Board, Team};
use crate::card::Card;
use crate::interface::{ask_card, ask_chips_bet, ask_gamble, ask_gamble_color, ask_number_of_played_cards, display_board, display_end_game, display_message};
use crate::player::Player;

mod player;
mod card;
mod board;
mod interface;

const NB_JOUEURS: usize = 4;
const CARTES_PAR_JOUEUR: usize = 5;
const TOTAL_CARTE: usize = 20;
const TOURS: usize = 3;
const VICTOIRE: i32 = 1;
const DEFAITE: i32 = 0;
const NB_JETONS: usize = 20;
const BLACK: i32 = 0;
const RED: i32 = 1;
const MULTICOLOR: i32 = 2;


fn main() {
    let mut board = Board::new();
    let mut players: Vec<Player> = Vec::new();
    for i in 1..NB_JOUEURS + 1 {
        players.push(Player::new(i));
    }

    initialiser_jeu(&mut board, &mut players);
    distribuer_cartes(&mut players);
    distribuer_jetons(&mut players);
    faire_pari(&mut players);

    //display_board(&mut board);
    //LANCEMENT DES 3 TOURS
    for tour in 1..TOURS + 1 {
        println!("\n============================================================\n\t\t\t>>> TOUR {} <<<", tour);
        placer_cartes_et_comparer(&mut board, &mut players);
        afficher_resultats_paris(&board, &players);
    }

    display_end_game(&mut board);
}

fn initialiser_jeu(plateau_jeu: &mut Board, players: &mut Vec<Player>) {
    for team_id in 0..2 {
        plateau_jeu.add_team(Team::new(team_id));
    }

    plateau_jeu.add_player_to_team(players[0].clone(), 0);
    plateau_jeu.add_player_to_team(players[1].clone(), 1);
    plateau_jeu.add_player_to_team(players[2].clone(), 0);
    plateau_jeu.add_player_to_team(players[3].clone(), 1);

    display_message("============================================================");
    display_message("\n          BIENVENUE SUR LE JEU DU POKER UNIVERSEL !         \n");
    display_message("============================================================");
    display_message("                LES ÉQUIPES ONT ÉTÉ FORMÉES !               \n");
    display_message("Équipe 1 : Joueur 1 et Joueur 3\n");
    display_message("Équipe 2 : Joueur 2 et Joueur 4\n");
    display_message("============================================================");
}

fn distribuer_cartes(players: &mut Vec<Player>) {
    let mut rng = rand::rng();
    let mut paquet: Vec<Card> = Vec::new();
    let mut carte_id: usize = 0;

    for valeur in 1..6 {
        for _i in 0..3 {
            paquet.push(Card::new(carte_id, valeur, BLACK));
            carte_id += 1;
        }
        for _i in 0..3 {
            paquet.push(Card::new(carte_id, valeur, RED));
            carte_id += 1;
        }
    }

    for i in 0..paquet.len() {
        let j = rng.random_range(0..paquet.len());
        paquet.swap(i, j);
    }

    for i in 0..NB_JOUEURS {
        for _j in 0..CARTES_PAR_JOUEUR {
            players[i].add_card_to_hand(paquet.remove(0));
        }
    }


    display_message("Les cartes ont été distribuées à tous les joueurs.");
}

fn distribuer_jetons(players: &mut Vec<Player>) {
    for i in 0..NB_JOUEURS {
        players[i].set_chips(NB_JETONS);
    }
}

fn faire_pari(players: &mut Vec<Player>) {
    display_message("\n============================================================");
    display_message("               >>> PREMIER TOUR DE PARIS <<<             ");
    display_message("============================================================");

    for player in &mut *players {
        let pari = ask_gamble(player);

        if pari == VICTOIRE {
            println!("le joueur {} à parié sur VICTOIRE", player.get_player_id());
        } else {
            println!("le joueur {} à parié sur DEFAITE", player.get_player_id());
        }

        let color = ask_gamble_color(player);
        if color == BLACK {
            println!("le joueur {} à parié sur NOIR", player.get_player_id());
        } else if pari == RED {
            println!("le joueur {} à parié sur RED", player.get_player_id());
        } else {
            println!("le joueur {} à parié sur MULTICOLOR", player.get_player_id());
        }

        miser_jetons(player);
    }

    display_message("\n============================================================");
    display_message("               >>> DEUXIEME TOUR DE PARIS <<<                 ");
    display_message("================================================================");

    for player in players {
        let pari = ask_gamble(player);
        player.set_slate(pari);

        if pari == VICTOIRE {
            println!("le joueur {} à parié sur VICTOIRE", player.get_player_id());
        } else {
            println!("le joueur {} à parié sur DEFAITE", player.get_player_id());
        }

        let color = ask_gamble_color(player);
        player.set_color_slate(color);
        if color == BLACK {
            println!("le joueur {} à parié sur NOIR", player.get_player_id());
        } else if pari == RED {
            println!("le joueur {} à parié sur RED", player.get_player_id());
        } else {
            println!("le joueur {} à parié sur MULTICOLOR", player.get_player_id());
        }


        miser_jetons(player);
    }
}

pub fn miser_jetons(player: &mut Player) {
    let mise = ask_chips_bet(player);
    player.play_chips(mise as usize);
    println!("le joueur {} à misé  {} jetons --> reste {}", player.get_player_id(), mise, player.get_chips());
}

pub fn placer_cartes_et_comparer(board: &mut Board, players: &mut Vec<Player>) {
    display_message("\n============================================================");
    display_message("            >>> PHASE DE PLACEMENT DES CARTES <<<             ");
    display_message("============================================================");

    for player in players.iter_mut()
    {
        let nb_cartes = ask_number_of_played_cards(player);
        for _i in 0..nb_cartes {
            let carte_index = ask_card(player);
            player.play_card(carte_index);
            let card_value = player.get_played_card(player.get_played_size() - 1).unwrap().get_value();
            player.set_score(player.get_score() + card_value);
        }
    }

    definir_gagnant_tour(players);
    attribuer_jetons(players);

    //le score de la team correspond a la somme des jetons des joueurs
    let score_team_1: usize = players[0].get_chips() + players[2].get_chips();
    let score_team_2: usize = players[1].get_chips() + players[3].get_chips();
    board.set_score_of_team(0, score_team_1);
    board.set_score_of_team(1, score_team_2);

    // Affichage des scores des équipes
    display_message("\n============================================================");
    display_message("                     >>> SCORES <<<                        ");
    display_message("============================================================");
    println!("jetons équipe 1 : {} ", board.get_score_of_team(0));
    println!("jetons équipe 2 : {} ", board.get_score_of_team(1));
    display_message("============================================================");

    for player in players.iter_mut() {
        let nb_card = player.get_played_size();
        for i in (0..nb_card).rev() {  // Parcours inversé pour éviter les décalages d'indices
            if let Some(card) = player.get_played_card(i) {
                board.add_out_of_game_card(card.clone());
                player.remove_played_card(i);
            }
        }
    }
}
fn definir_gagnant_tour(players: &mut Vec<Player>) {
    // Initialiser les scores par équipe et par couleur
    let mut score_team_1_black: usize = 0;
    let mut score_team_1_red: usize = 0;
    let mut score_team_1_total: usize = 0;

    let mut score_team_2_black: usize = 0;
    let mut score_team_2_red: usize = 0;
    let mut score_team_2_total: usize = 0;

    // Calculer les scores par couleur pour chaque équipe
    for (i, player) in players.iter().enumerate() {
        let is_team_1 = i == 0 || i == 2;

        // Compter les points par couleur
        for card_idx in 0..player.get_played_size() {
            if let Some(card) = player.get_played_card(card_idx) {
                let card_value = card.get_value();
                let card_color = card.get_color();

                if is_team_1 {
                    score_team_1_total += card_value;
                    if card_color == BLACK {
                        score_team_1_black += card_value;
                    } else if card_color == RED {
                        score_team_1_red += card_value;
                    }
                } else {
                    score_team_2_total += card_value;
                    if card_color == BLACK {
                        score_team_2_black += card_value;
                    } else if card_color == RED {
                        score_team_2_red += card_value;
                    }
                }
            }
        }
    }

    // Déterminer le gagnant pour chaque type de pari de couleur
    let team_1_wins_black = score_team_1_black > score_team_2_black;
    let team_1_wins_red = score_team_1_red > score_team_2_red;
    let team_1_wins_multicolor = score_team_1_total > score_team_2_total;

    // Afficher les scores par couleur
    println!("Score équipe 1 - Cartes noires: {}", score_team_1_black);
    println!("Score équipe 1 - Cartes rouges: {}", score_team_1_red);
    println!("Score équipe 1 - Total: {}", score_team_1_total);
    println!("Score équipe 2 - Cartes noires: {}", score_team_2_black);
    println!("Score équipe 2 - Cartes rouges: {}", score_team_2_red);
    println!("Score équipe 2 - Total: {}", score_team_2_total);

    // Réinitialiser les scores des joueurs pour le prochain tour
    for player in players.iter_mut() {
        player.set_score(0);
        player.set_winner(0);
    }

    // Déterminer les gagnants des paris en fonction de la couleur choisie
    for (i, player) in players.iter_mut().enumerate() {
        let is_team_1 = i == 0 || i == 2;
        let predicted_win = player.get_slate() == VICTOIRE;
        let color_choice = player.get_color_slate();

        // Déterminer si l'équipe du joueur a gagné selon son pari de couleur
        let team_wins = match color_choice {
            color if color == BLACK => {
                if is_team_1 { team_1_wins_black } else { !team_1_wins_black }
            },
            color if color == RED => {
                if is_team_1 { team_1_wins_red } else { !team_1_wins_red }
            },
            _ => { // MULTICOLOR
                if is_team_1 { team_1_wins_multicolor } else { !team_1_wins_multicolor }
            }
        };

        // Un joueur gagne si sa prédiction (victoire/défaite) correspond au résultat
        let player_wins = (predicted_win && team_wins) || (!predicted_win && !team_wins);

        if player_wins {
            player.set_winner(1);
            // Les jetons seront attribués dans attribuer_jetons
        }
    }
}

fn attribuer_jetons(players: &mut Vec<Player>) {
    players.iter_mut().for_each(|player| {
        if player.get_winner() == 1 {
            let chips_winned = player.get_played_chips() * 2;
            player.set_chips(player.get_chips() + chips_winned);
        }
        player.set_played_chips(0);
    })
}

fn afficher_resultats_paris(plateau_jeu: &Board, players: &Vec<Player>) {
    display_message("\n============================================================");
    display_message("                  >>> RÉSULTATS DES PARIS <<<                ");
    display_message("============================================================");

    // Vérifier les paris de chaque joueur
    for i in 0..NB_JOUEURS {
        let equipe_joueur = if i % 2 == 0 { 0 } else { 1 };
        let player = &players[i];
        let pari = player.get_slate();
        let a_gagne = player.get_winner() == 1;

        let resultat_pari = if a_gagne {
            format!(
                "\u{001B}[0;32m>>> Le Joueur {} a gagné son pari ! <<<\u{001B}[0m\n\
                Pari: {} | couleur : {} | Jetons actuels: {}\n",
                player.get_player_id(),
                if pari == VICTOIRE { "VICTOIRE" } else { "DÉFAITE" },
                if player.get_color_slate() == BLACK { "NOIR" } else if player.get_color_slate() == RED { "ROUGE" } else { "MULTICOLOR" },
                player.get_chips()
            )
        } else {
            format!(
                "\u{001B}[0;31m>>> Le Joueur {} a perdu son pari. <<<\u{001B}[0m\n\
                Pari: {} | couleur : {} | Jetons actuels: {}\n",
                player.get_player_id(),
                if pari == VICTOIRE { "VICTOIRE" } else { "DÉFAITE" },
                if player.get_color_slate() == BLACK { "NOIR" } else if player.get_color_slate() == RED { "ROUGE" } else { "MULTICOLOR" },
                player.get_chips()
            )
        };
        display_message(&resultat_pari);
    }

    // Afficher les totaux par équipe
    let jetons_equipe1 = players[0].get_chips() + players[2].get_chips();
    let jetons_equipe2 = players[1].get_chips() + players[3].get_chips();

    display_message("\n============================================================");
    display_message("                 >>> TOTAL DES JETONS <<<                   ");
    display_message("============================================================");

    let total_jetons = format!(
        "\u{001B}[0;36mÉquipe 1 : {} jetons\u{001B}[0m\n\
        \u{001B}[0;32mÉquipe 2 : {} jetons\u{001B}[0m",
        jetons_equipe1, jetons_equipe2
    );
    display_message(&total_jetons);
}