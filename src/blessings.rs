use crate::{
    actions::{
        add_card_to_master_deck::AddCardToMasterDeckAction, gain_potion::GainPotionAction,
        increase_max_hp::IncreaseMaxHPAction,
    },
    cards::random_uncommon_colorless,
    game::{Game, RemoveFromMasterGameState, RunActionsGameState, TransformMasterGameState},
    potion::random_common_potion,
    relic::random_common_relic,
    state::GameState,
    step::Step,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Blessing {
    GainMaxHPSmall,
    CommonRelic,
    RemoveRelic, // just to prevent dead code warnings
    TransformOne,
    RemoveOne,
    RandomUncommonColorless,
    RandomPotion,
}

impl Blessing {
    pub fn run(&self, game: &mut Game) {
        use Blessing::*;
        match self {
            GainMaxHPSmall => {
                game.action_queue.push_bot(IncreaseMaxHPAction(8));
            }
            CommonRelic => {
                let r = random_common_relic(&mut game.rng);
                game.add_relic(r);
            }
            RemoveRelic => {
                game.remove_relic(game.relics[0].get_class());
            }
            TransformOne => {
                game.state.push_state(TransformMasterGameState);
            }
            RemoveOne => {
                game.state.push_state(RemoveFromMasterGameState);
            }
            RandomUncommonColorless => {
                let r = random_uncommon_colorless(&mut game.rng);
                game.action_queue.push_bot(AddCardToMasterDeckAction(r));
            }
            RandomPotion => {
                let p = random_common_potion(&mut game.rng);
                game.action_queue.push_bot(GainPotionAction(p));
            }
        }
    }
}

#[derive(Debug)]

pub struct ChooseBlessingGameState;

impl GameState for ChooseBlessingGameState {
    fn valid_steps(&self, _: &Game) -> Option<Vec<Box<dyn Step>>> {
        Some(vec![
            Box::new(ChooseBlessingStep(Blessing::GainMaxHPSmall)),
            Box::new(ChooseBlessingStep(Blessing::CommonRelic)),
            Box::new(ChooseBlessingStep(Blessing::RemoveRelic)),
            Box::new(ChooseBlessingStep(Blessing::TransformOne)),
            Box::new(ChooseBlessingStep(Blessing::RemoveOne)),
            Box::new(ChooseBlessingStep(Blessing::RandomUncommonColorless)),
            Box::new(ChooseBlessingStep(Blessing::RandomPotion)),
        ])
    }
}

#[derive(Eq, PartialEq, Debug)]
struct ChooseBlessingStep(Blessing);

impl Step for ChooseBlessingStep {
    fn run(&self, game: &mut Game) {
        self.0.run(game);
        game.state.push_state(RunActionsGameState);
    }

    fn description(&self, _: &Game) -> String {
        format!("{:?}", self.0)
    }
}
