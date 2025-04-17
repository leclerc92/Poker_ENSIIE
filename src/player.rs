use crate::card::Card;

const MAX_HAND_SIZE: usize = 5;
const MAX_PLAYER_SIZE: usize = 5;
const MAX_PLAYERS: usize = 4;

#[derive(Debug, Clone)]
pub struct Player {
    id: usize,
    hand: Vec<Card>,
    played_cards: Vec<Card>,
    slate: i32,
    color_slate: i32,
    chips: usize,
    played_chips: usize,
    score: usize,
    winner: usize,
}

impl Player {
    pub fn new(id: usize) -> Self {
        Player {
            id,
            hand: Vec::new(),
            played_cards: Vec::new(),
            slate: 0,
            chips: 0,
            played_chips: 0,
            score: 0,
            winner: 0,
            color_slate: 3,
        }
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        if self.hand.len() + 1 > MAX_HAND_SIZE {
            println!("Warning: hand {} too long", MAX_HAND_SIZE);
            return;
        }
        self.hand.push(card);
    }

    pub fn remove_card_from_hand(&mut self, card_index: usize) {
        if card_index < self.hand.len() {
            self.hand.remove(card_index);
        }
    }

    pub fn play_card(&mut self, card_index: usize) {
        if self.played_cards.len() >= MAX_PLAYER_SIZE {
            println!("Erreur la table est pleine ");
            return;
        }
        let card = self.hand.remove(card_index);
        self.played_cards.push(card);
    }

    pub fn remove_played_card(&mut self, card_index: usize) {
        if card_index < self.played_cards.len() {
            self.played_cards.remove(card_index);
        }
    }

    pub fn get_player_id(&self) -> usize {
        self.id
    }

    pub fn get_size_of_hand(&self) -> usize {
        self.hand.len()
    }

    pub fn get_card_in_hand(&self, card_index: usize) -> Option<&Card> {
        self.hand.get(card_index)
    }

    pub fn get_played_size(&self) -> usize {
        self.played_cards.len()
    }

    pub fn get_played_card(&self, card_index: usize) -> Option<&Card> {
        self.played_cards.get(card_index)
    }

    pub fn get_slate(&self) -> i32 {
        self.slate
    }

    // SETTERS
    pub fn set_slate(&mut self, pari: i32) {
        self.slate = pari;
    }

    pub fn get_chips(&self) -> usize {
        self.chips
    }

    pub fn set_chips(&mut self, count: usize) {
        self.chips = count;
    }

    pub fn play_chips(&mut self, count: usize) {
        if self.chips >= count {
            self.played_chips += count;
            self.chips -= count;
        }
    }

    pub fn set_played_chips(&mut self, count: usize) {
        if self.chips >= count {}
    }

    pub fn get_played_chips(&self) -> usize {
        self.played_chips
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn set_score(&mut self, score: usize) {
        self.score = score;
    }

    pub fn get_winner(&self) -> usize {
        self.winner
    }

    pub fn set_winner(&mut self, is_winner: usize) {
        self.winner = is_winner;
    }

    pub fn get_color_slate(&self) -> i32 {
        self.color_slate
    }

    pub fn set_color_slate(&mut self, slate: i32) {
        self.color_slate = slate;
    }
}