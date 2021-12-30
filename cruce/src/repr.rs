use derive_more::{BitOr, BitAnd, BitXor, From};

#[repr(u8)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum Suit {
    Acorns = 1,
    Leaves = 2,
    Hearts = 3,
    Bells = 4
}

impl From<Suit> for usize {
    fn from(item: Suit) -> usize {
        (item as u8 - 1) as usize
    }
}

impl From<u8> for Suit {
    fn from(item: u8) -> Suit {
        match item {
            1 => Suit::Acorns,
            2 => Suit::Leaves,
            3 => Suit::Hearts,
            4 => Suit::Bells,
            _ => unreachable!()
        }
    }
}

#[repr(u8)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum Rank {
    Nine = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Ten = 4,
    Ace = 5
}

impl From<u8> for Rank {
    fn from(item: u8) -> Rank {
        match item {
            0 => Rank::Nine,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Ten,
            5 => Rank::Ace,
            _ => unreachable!()
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub struct Card(pub(crate) u8);

#[derive(BitOr, BitXor, BitAnd, From, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)] 
pub struct CardSet(pub(crate) u32);

impl Card {

    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card(rank as u8 * 4 + suit as u8 - 1)
    }
        
    pub fn to_set(self) -> CardSet {
        CardSet(1 << self.0) 
    }
    
    pub fn points(self) -> u8 {
        match self.rank() {
            Rank::Nine => 0,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Ten => 10,
            Rank::Ace => 11
        }
    }

    pub fn suit(self) -> Suit {
        (self.0 % 4 + 1).into()
    }

    pub fn rank(self) -> Rank {
        (self.0 / 4).into()
    }
}

pub struct CardIterator {
    set: CardSet
}

impl Iterator for CardIterator {
    type Item = Card;
    
    fn next(&mut self) -> Option<Card> {
        if self.set.is_empty() {
            return None;
        }

        let set : u32 = self.set.0;
        let card = Card( (((set - 1) & set) ^ set).trailing_zeros() as u8 );

        self.set = CardSet((set - 1) & set);

        Some(card)
    }
}

impl CardSet {

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn contains(self, card: Card) -> bool {
        !(self & card.to_set()).is_empty()
    }
}

impl IntoIterator for CardSet {
    type Item = Card;
    type IntoIter = CardIterator;

    fn into_iter(self) -> CardIterator {
        CardIterator {
            set: self
        }
    }
}


