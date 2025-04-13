use crate::card::Card;
use crate::player::Player;

pub(crate) struct Team {
    id: usize,
    score: usize,
    nb_players: usize,
    list_players: Vec<Player>,
}

pub(crate) struct Board {
    list_teams: Vec<Team>,
    list_cards_out: Vec<Card>,
}

impl Team {
    pub fn new(id: usize) -> Self {
        Team {
            id,
            score: 0,
            nb_players: 0,
            list_players: Vec::new(),
        }
    }

    // Mettre à jour nb_players quand on ajoute un joueur
    pub fn add_player(&mut self, player: Player) {
        self.list_players.push(player);
        self.nb_players = self.list_players.len();
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            list_teams: Vec::new(),
            list_cards_out: Vec::new(),
        }
    }

    pub fn add_team(&mut self, team: Team) {
        self.list_teams.push(team);
    }

    pub fn get_number_of_teams(&self) -> usize {
        self.list_teams.len()
    }

    pub fn get_number_of_players_in_team(&self, team_id: usize) -> usize {
        self.list_teams[team_id].nb_players
    }

    pub fn get_player(&self, team_id: usize, player_id: usize) -> Option<&Player> {
        self.list_teams[team_id].list_players.get(player_id)
    }

    pub fn get_score_of_team(&self, team_id: usize) -> usize {
        self.list_teams[team_id].score
    }

    pub fn set_score_of_team(&mut self, team_id: usize, score: usize) {
        self.list_teams[team_id].score = score;
    }

    pub fn add_out_of_game_card(&mut self, card: Card) {
        self.list_cards_out.push(card);
    }

    pub fn get_number_of_out_of_game_cards(&self) -> usize {
        self.list_cards_out.len()
    }

    pub fn get_out_of_game_card(&self, card_index: usize) -> Option<&Card> {
        self.list_cards_out.get(card_index)
    }

    pub fn add_player_to_team(&mut self, player: Player, team_id: usize) {
        if team_id < self.list_teams.len() {
            self.list_teams[team_id].add_player(player);
        }
    }

    pub fn remove_out_of_game_card(&mut self, card_index: usize) {
        self.list_cards_out.remove(card_index);
    }
}