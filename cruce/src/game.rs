use crate::repr::{Card, CardSet, Suit};
use crate::constant::SUIT;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GameState {
    player: u8, // 1
    turn: u8, // 1
    points: [u8; 2], // 2
    discard: CardSet, // 4
    played: [Card; 4], // 4
    hands: [CardSet; 4], //16
    trump: Option<Suit> // 1
} // total 29 bytes

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LegalCard(Card);

impl LegalCard {
    pub fn as_card(self) -> Card {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LegalCardSet(CardSet);

impl LegalCardSet {
    pub fn as_set(self) -> CardSet {
        self.0
    }
}

struct LegalCardIterator {
    set: LegalCardSet
}

impl Iterator for LegalCardIterator {
    type Item = LegalCard;
    
    fn next(&mut self) -> Option<LegalCard> {
        if self.set.as_set().is_empty() {
            return None;
        }

        let set : u32 = self.set.as_set().0;
        let card = LegalCard( Card((((set - 1) & set) ^ set).trailing_zeros() as u8 ));

        self.set = LegalCardSet(CardSet((set - 1) & set));

        Some(card)
    }
}


impl GameState {
    
    pub fn new(hands: [CardSet; 4]) -> Self {
        GameState {
            player: 0,
            turn: 0,
            points: [0; 2],
            discard: CardSet(0),
            played: [Card(31); 4],
            hands,
            trump: None
        }
    }

    pub fn check(self, card: Card) -> Option<LegalCard> {
        match self.legal().as_set().contains(card) {
            true => Some(LegalCard(card)),
            false => None
        }
    }

    pub fn legal(self) -> LegalCardSet {

        let hand = self.hands[self.player as usize];

        if self.turn % 4 == 0 {
            return LegalCardSet(hand);
        }
        
        let first = self.played[0];
        let suit = first.suit();
        let trump = self.trump.unwrap();
        
        let same_suit = hand & SUIT[<usize as From<Suit>>::from(suit)];
        
        if !same_suit.is_empty() {
            return LegalCardSet(same_suit);
        }

        let trump_suit = hand & SUIT[<usize as From<Suit>>::from(trump)];
        
        if !trump_suit.is_empty() {
            LegalCardSet(trump_suit)
        } else {
            LegalCardSet(hand)
        }
    }

    pub fn play(mut self, legal_card: LegalCard) -> GameState {
        let card = legal_card.0;
        
        let hand = self.hands[self.player as usize];

        self.hands[self.player as usize] =  hand ^ card.to_set();
        self.played[(self.turn % 4) as usize] = card;
        
        self.player = (self.player + 1) % 4;
        
        if self.turn == 0 {
            self.trump = Some(card.suit());
        }

        self.turn += 1;
                
        self
    }

    fn take(mut self) -> GameState {
        let mut player = self.player;
        let trump = self.trump.unwrap();
        
        let mut best_card = self.played[0];
        let mut best_player = player;
        player = (player + 1) % 4;
        let mut points = 0u8;
        let mut call = 0;
        for &card in &self.played[1..4] {
            points += card.points();
            if card.suit() == best_card.suit() && card.rank() > best_card.rank() {
                best_card = card;
                best_player = player;
            } else if card.suit() == trump {
                best_card = card;
                best_player = player;
            }

            player = (player + 1) % 4;
        }
        
        self.points[(best_player%2) as usize] += points;

        self
    }
}
