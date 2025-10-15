use crate::{
    card::CardRef,
    cards::{CardClass, CardType},
};

#[derive(Debug)]
pub enum GameState {
    RunActions,
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
    CombatEnd,
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
        is_free: bool,
    },
    Defeat,
    Victory,
}

pub struct GameStateManager {
    stack: Vec<GameState>,
    debug: bool,
}

impl std::fmt::Debug for GameStateManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "state stack: {:?}", self.stack)
    }
}

impl GameStateManager {
    pub fn new(state: GameState) -> Self {
        Self {
            stack: vec![state],
            debug: false,
        }
    }
    pub fn set_debug(&mut self) {
        self.debug = true;
    }
    pub fn cur_state(&self) -> &GameState {
        self.stack.last().unwrap()
    }
    pub fn cur_state_mut(&mut self) -> &mut GameState {
        self.stack.last_mut().unwrap()
    }
    pub fn push_state(&mut self, state: GameState) {
        if self.debug {
            println!("push_state {:?}", state);
        }
        self.stack.push(state);
    }
    fn pop_state_impl(&mut self, check_not_empty: bool) {
        let state = self.stack.pop().unwrap();
        if self.debug {
            println!("pop_state {:?}", state);
        }
        if check_not_empty {
            assert!(!self.stack.is_empty());
        }
    }
    pub fn pop_state(&mut self) {
        self.pop_state_impl(true);
    }
    pub fn set_state(&mut self, state: GameState) {
        self.pop_state_impl(false);
        self.push_state(state);
    }
}
