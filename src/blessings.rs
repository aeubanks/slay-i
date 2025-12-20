use crate::{
    actions::{
        add_card_to_master_deck::AddCardToMasterDeckAction, gain_potion::GainPotionAction,
        gain_relic::GainRelicAction, increase_max_hp::IncreaseMaxHPAction,
        remove_relic::RemoveRelicAction,
    },
    cards::random_uncommon_colorless,
    game::{ChooseRemoveFromMasterGameState, Game, RunActionsGameState, TransformMasterGameState},
    potion::random_common_potion,
    relic::random_common_relic,
    state::{GameState, Steps},
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
                game.action_queue.push_bot(GainRelicAction(r));
            }
            RemoveRelic => {
                let r = game.relics[0].get_class();
                game.action_queue.push_bot(RemoveRelicAction(r));
            }
            TransformOne => {
                game.state.push_state(TransformMasterGameState);
            }
            RemoveOne => {
                game.state.push_state(ChooseRemoveFromMasterGameState);
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
    fn valid_steps(&self, _: &Game) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.push(ChooseBlessingStep(Blessing::GainMaxHPSmall));
        steps.push(ChooseBlessingStep(Blessing::CommonRelic));
        steps.push(ChooseBlessingStep(Blessing::RemoveRelic));
        steps.push(ChooseBlessingStep(Blessing::TransformOne));
        steps.push(ChooseBlessingStep(Blessing::RemoveOne));
        steps.push(ChooseBlessingStep(Blessing::RandomUncommonColorless));
        steps.push(ChooseBlessingStep(Blessing::RandomPotion));
        Some(steps)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ChooseBlessingStep(pub Blessing);

impl Step for ChooseBlessingStep {
    fn should_pop_state(&self) -> bool {
        true
    }

    fn run(&self, game: &mut Game) {
        self.0.run(game);
        game.state.push_state(RunActionsGameState);
    }

    fn description(&self, _: &Game) -> String {
        format!("{:?}", self.0)
    }
}
