use rand::Rng;
use crate::board::{Board, Team};
use crate::card::Card;
use crate::interface::{ask_chips_bet, ask_gamble, display_board, display_message};
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

    //LANCEMENT DES 3 TOURS
    for tour in 1..TOURS + 1 {
        println!("\n============================================================\n\t\t\t>>> TOUR {} <<<", tour);
        display_board(&mut board);
    }
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
        for _i in 0..5 {
            paquet.push(Card::new(carte_id, valeur));
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
    }

    display_message("\n============================================================");
    display_message("               >>> DEUXIEME TOUR DE PARIS <<<                 ");
    display_message("================================================================");

    for player in players {
        let pari = ask_gamble(player);
        player.set_slate(pari as usize);

        if pari == VICTOIRE {
            println!("le joueur {} à parié sur VICTOIRE", player.get_player_id());
        } else {
            println!("le joueur {} à parié sur DEFAITE", player.get_player_id());
        }

        miser_jetons(player);
    }
}

pub fn miser_jetons(player: &mut Player) {
    let mise = ask_chips_bet(player);
    player.play_chips(mise as usize);
    println!("le joueur {} à misé  {} jetons --> reste {}", player.get_player_id(), mise, player.get_chips());
}