use crate::{
    card::CardRef,
    cards::{CardClass, CardType},
};

#[derive(Debug)]
pub enum GameState {
    Blessing,
    RemoveCard,
    TransformCard,
    RollCombat,
    CombatBegin,
    PlayerTurnBegin,
    PlayerTurn,
    PlayerTurnEnd,
    MonsterTurn,
    EndOfRound,
    Armaments,
    Memories {
        num_cards_remaining: i32,
        cards_to_memories: Vec<CardRef>,
    },
    ExhaustOneCardInHand,
    ExhaustCardsInHand {
        num_cards_remaining: i32,
        cards_to_exhaust: Vec<CardRef>,
    },
    Gamble {
        cards_to_gamble: Vec<CardRef>,
    },
    PlaceCardInHandOnTopOfDraw,
    PlaceCardInDiscardOnTopOfDraw,
    Exhume,
    DualWield(i32),
    FetchCardFromDraw(CardType),
    ForethoughtAny {
        cards_to_forethought: Vec<CardRef>,
    },
    ForethoughtOne,
    Discovery {
        classes: Vec<CardClass>,
        amount: i32,
    },
    Defeat,
    Victory,
}

#[derive(Debug)]
pub struct GameStateManager {
    stack: Vec<GameState>,
}

impl GameStateManager {
    pub fn new(state: GameState) -> Self {
        Self { stack: vec![state] }
    }
    pub fn cur_state(&self) -> &GameState {
        self.stack.last().unwrap()
    }
    pub fn cur_state_mut(&mut self) -> &mut GameState {
        self.stack.last_mut().unwrap()
    }
    pub fn push_state(&mut self, state: GameState) {
        self.stack.push(state);
    }
    pub fn pop_state(&mut self) {
        self.stack.pop().unwrap();
        assert!(!self.stack.is_empty());
    }
    pub fn set_state(&mut self, state: GameState) {
        self.stack.pop().unwrap();
        self.push_state(state);
    }
}
