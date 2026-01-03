use crate::{
    action::Action,
    actions::gain_status::GainStatusAction,
    game::{CreatureRef, Game},
    status::Status,
};

pub struct RobAction {
    pub source: CreatureRef,
    pub amount: i32,
}

impl Action for RobAction {
    fn run(&self, game: &mut Game) {
        let amount = game.gold.min(self.amount);
        if amount != 0 {
            game.gold -= amount;
            game.action_queue.push_top(GainStatusAction {
                status: Status::StolenGold,
                amount,
                target: self.source,
            });
        }
    }
}

impl std::fmt::Debug for RobAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rob {}", self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cards::CardClass,
        game::{CreatureRef, GameBuilder},
        rewards::StolenGoldRewardStep,
        status::Status,
    };

    #[test]
    fn test_rob() {
        let mut g = GameBuilder::default().build_combat();

        g.run_action(RobAction {
            source: CreatureRef::monster(0),
            amount: 10,
        });
        assert!(!g.monsters[0].creature.has_status(Status::StolenGold));

        g.gold = 5;
        g.run_action(RobAction {
            source: CreatureRef::monster(0),
            amount: 10,
        });
        assert_eq!(g.gold, 0);
        assert_eq!(
            g.monsters[0].creature.get_status(Status::StolenGold),
            Some(5)
        );

        g.gold = 15;
        g.run_action(RobAction {
            source: CreatureRef::monster(0),
            amount: 10,
        });
        assert_eq!(
            g.monsters[0].creature.get_status(Status::StolenGold),
            Some(15)
        );
        assert_eq!(g.gold, 5);

        g.play_card(CardClass::DebugKillAll, None);
        g.step_test(StolenGoldRewardStep);
        assert_eq!(g.gold, 20);
    }
}
