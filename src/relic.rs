use lazy_static::lazy_static;

use crate::{
    actions::{
        block::BlockAction, damage::DamageAction, draw::DrawAction, gain_status::GainStatusAction,
        heal::HealAction, play_card::PlayCardAction,
    },
    cards::CardType,
    game::{CreatureRef, Rand},
    queue::ActionQueue,
    rng::rand_slice,
    status::Status,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum RelicRarity {
    Starter,
    Common,
    Uncommon,
    Rare,
    Shop,
    Event,
    Boss,
}

macro_rules! r {
    ($($name:ident => $rarity:expr),+,) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum RelicClass {
            $(
                $name,
            )+
        }
        impl RelicClass {
            pub fn rarity(&self) -> RelicRarity {
                use RelicRarity::*;
                match self {
                    $(Self::$name => $rarity,)+
                }
            }
        }
        impl RelicClass {
            pub fn all() -> Vec<Self> {
                vec![$(Self::$name,)+]
            }
        }
    };
}

r!(
    BurningBlood => Starter,

    Akabeko => Common, // TODO
    Anchor => Common,
    AncientTeaSet => Common, // TODO
    ArtOfWar => Common, // TODO
    BagOfMarbles => Common, // TODO
    BagOfPrep => Common,
    BloodVial => Common,
    BronzeScales => Common, // TODO
    CentennialPuzzle => Common, // TODO
    CeramicFish => Common, // TODO
    DreamCatcher => Common, // TODO
    HappyFlower => Common, // TODO
    JuzuBracelet => Common, // TODO
    Lantern => Common, // TODO
    MawBank => Common, // TODO
    MealTicket => Common, // TODO
    Nunchaku => Common, // TODO
    OddlySmoothStone => Common, // TODO
    Omamori => Common, // TODO
    Orichalcum => Common, // TODO
    PenNib => Common, // TODO
    PotionBelt => Common, // TODO
    PreservedInsect => Common, // TODO
    RegalPillow => Common, // TODO
    SmilingMask => Common, // TODO
    Strawberry => Common, // TODO
    Boot => Common, // TODO
    TinyChest => Common, // TODO
    ToyOrnithopter => Common, // TODO
    Vajra => Common, // TODO
    WarPaint => Common, // TODO
    Whetstone => Common, // TODO
    RedSkull => Common, // TODO

    BlueCandle => Uncommon,
    BottledFlame => Uncommon, // TODO
    BottledLightning => Uncommon, // TODO
    BottledTornado => Uncommon, // TODO
    DarkstonePeriapt => Uncommon, // TODO
    EternalFeather => Uncommon, // TODO
    FrozenEgg => Uncommon, // TODO
    GremlinHorn => Uncommon, // TODO
    HornCleat => Uncommon,
    InkBottle => Uncommon, // TODO
    Kunai => Uncommon,
    LetterOpener => Uncommon, // TODO
    Matryoshka => Uncommon, // TODO
    MeatOnTheBone => Uncommon, // TODO
    MercuryHourglass => Uncommon, // TODO
    MoltenEgg => Uncommon, // TODO
    MummifiedHand => Uncommon, // TODO
    OrnamentalFan => Uncommon, // TODO
    Pantograph => Uncommon, // TODO
    Pear => Uncommon, // TODO
    QuestionCard => Uncommon, // TODO
    Shruiken => Uncommon,
    SingingBowl => Uncommon, // TODO
    StrikeDummy => Uncommon, // TODO
    Sundial => Uncommon, // TODO
    TheCourier => Uncommon, // TODO
    ToxicEgg => Uncommon, // TODO
    WhiteBeastStatue => Uncommon, // TODO
    PaperPhrog => Uncommon, // TODO
    SelfFormingClay => Uncommon, // TODO

    BirdFacedUrn => Rare, // TODO
    Calipers => Rare, // TODO
    CaptainsWheel => Rare,
    DeadBranch => Rare, // TODO
    DuVuDoll => Rare, // TODO
    FossilizedHelix => Rare, // TODO
    GamblingChip => Rare, // TODO
    Ginger => Rare, // TODO
    Girya => Rare, // TODO
    IceCream => Rare, // TODO
    IncenseBurner => Rare, // TODO
    LizardTail => Rare, // TODO
    Mango => Rare, // TODO
    OldCoin => Rare, // TODO
    PeacePipe => Rare, // TODO
    Pocketwatch => Rare, // TODO
    PrayerWheel => Rare, // TODO
    Shovel => Rare, // TODO
    StoneCalendar => Rare, // TODO
    ThreadAndNeedle => Rare, // TODO
    Torii => Rare, // TODO
    TungstenRod => Rare, // TODO
    Turnip => Rare, // TODO
    UnceasingTop => Rare, // TODO
    WingBoots => Rare, // TODO
    ChampionBelt => Rare, // TODO
    CharonsAshes => Rare, // TODO
    MagicFlower => Rare, // TODO

    Cauldron => Shop, // TODO
    ChemicalX => Shop, // TODO
    ClockworkSouvenir => Shop, // TODO
    DollysMirror => Shop, // TODO
    FrozenEye => Shop, // TODO
    HandDrill => Shop, // TODO
    LeesWaffle => Shop, // TODO
    MedicalKit => Shop,
    MembershipCard => Shop, // TODO
    OrangePellets => Shop, // TODO
    Orrery => Shop, // TODO
    // PrismaticShard => Shop, // not supported
    SlingOfCourage => Shop, // TODO
    StrangeSpoon => Shop, // TODO
    TheAbacus => Shop, // TODO
    Toolbox => Shop, // TODO
    Brimstone => Shop, // TODO

    Astrolabe => Boss, // TODO
    BlackStar => Boss, // TODO
    BustedCrown => Boss, // TODO
    CallingBell => Boss, // TODO
    CoffeeDripper => Boss, // TODO
    CursedKey => Boss, // TODO
    Ectoplasm => Boss, // TODO
    EmptyCage => Boss, // TODO
    FusionHammer => Boss, // TODO
    PandorasBox => Boss, // TODO
    PhilosophersStone => Boss, // TODO
    RunicDome => Boss, // TODO
    RunicPyramid => Boss, // TODO
    SacredBark => Boss,
    SlaversCollar => Boss, // TODO
    SneckoEye => Boss, // TODO
    Sozu => Boss, // TODO
    TinyHouse => Boss, // TODO
    VelvetChoker => Boss, // TODO
    BlackBlood => Boss, // TODO
    MarkOfPain => Boss, // TODO
    RunicCube => Boss, // TODO

    BloodyIdol => Event, // TODO
    CultistHeadpiece => Event, // TODO
    Enchiridion => Event, // TODO
    FaceOfCleric => Event, // TODO
    GoldenIdol => Event, // TODO
    GremlinVisage => Event, // TODO
    MarkOfTheBloom => Event, // TODO
    MutagenicStrength => Event, // TODO
    NlothsGift => Event, // TODO
    NlothsHungryFace => Event, // TODO
    Necronomicon => Event, // TODO
    NeowsLament => Event, // TODO
    NilryCodex => Event, // TODO
    OddMushroom => Event, // TODO
    RedMask => Event, // TODO
    SpiritPoop => Event, // TODO
    SsserpentHead => Event, // TODO
    WarpedTongs => Event, // TODO
);

type RelicCallback = fn(&mut i32, &mut ActionQueue);
type RelicCardCallback = fn(&mut i32, &mut ActionQueue, &PlayCardAction);

impl RelicClass {
    pub fn pre_combat(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            BloodVial => Some(blood_vial),
            HornCleat | CaptainsWheel => Some(set_value_zero),
            _ => None,
        }
    }
    pub fn combat_finish(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            BurningBlood => Some(burning_blood),
            _ => None,
        }
    }
    pub fn combat_start_pre_draw(&self) -> Option<RelicCallback> {
        None
    }
    pub fn combat_start_post_draw(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            BagOfPrep => Some(bag_of_prep),
            Anchor => Some(anchor),
            _ => None,
        }
    }
    pub fn on_card_played(&self) -> Option<RelicCardCallback> {
        use RelicClass::*;
        match self {
            BlueCandle => Some(blue_candle),
            Kunai => Some(kunai),
            Shruiken => Some(shruiken),
            _ => None,
        }
    }
    pub fn turn_start(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            Kunai | Shruiken => Some(set_value_zero),
            HornCleat => Some(horn_cleat),
            CaptainsWheel => Some(captains_wheel),
            _ => None,
        }
    }
    pub fn turn_end(&self) -> Option<RelicCallback> {
        None
    }
}

fn set_value_zero(v: &mut i32, _: &mut ActionQueue) {
    *v = 0;
}

fn inc_wrap(v: &mut i32, max: i32) -> bool {
    *v += 1;
    if *v == max {
        *v = 0;
        true
    } else {
        false
    }
}

fn kunai(v: &mut i32, queue: &mut ActionQueue, play_card: &PlayCardAction) {
    if play_card.card.borrow().class.ty() == CardType::Attack && inc_wrap(v, 3) {
        queue.push_bot(GainStatusAction {
            status: Status::Dexterity,
            amount: 1,
            target: CreatureRef::player(),
        });
    }
}

fn shruiken(v: &mut i32, queue: &mut ActionQueue, play_card: &PlayCardAction) {
    if play_card.card.borrow().class.ty() == CardType::Attack && inc_wrap(v, 3) {
        queue.push_bot(GainStatusAction {
            status: Status::Strength,
            amount: 1,
            target: CreatureRef::player(),
        });
    }
}

fn burning_blood(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(HealAction {
        target: CreatureRef::player(),
        amount: 6,
    });
}

fn blood_vial(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(HealAction {
        target: CreatureRef::player(),
        amount: 2,
    });
}

fn bag_of_prep(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(DrawAction(2));
}

fn anchor(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(BlockAction::player_flat_amount(10));
}

fn horn_cleat(v: &mut i32, queue: &mut ActionQueue) {
    *v += 1;
    if *v == 2 {
        queue.push_bot(BlockAction::player_flat_amount(14));
    }
}

fn captains_wheel(v: &mut i32, queue: &mut ActionQueue) {
    *v += 1;
    if *v == 3 {
        queue.push_bot(BlockAction::player_flat_amount(18));
    }
}

fn blue_candle(_: &mut i32, queue: &mut ActionQueue, play: &PlayCardAction) {
    if play.card.borrow().class.ty() == CardType::Curse {
        queue.push_bot(DamageAction::lose_hp(1, CreatureRef::player()));
    }
}

pub struct Relic {
    class: RelicClass,
    value: i32,
}

impl Relic {
    pub fn get_class(&self) -> RelicClass {
        self.class
    }
}

macro_rules! trigger {
    ($name:ident) => {
        pub fn $name(&mut self, queue: &mut ActionQueue) {
            if let Some(f) = self.class.$name() {
                f(&mut self.value, queue)
            }
        }
    };
}
macro_rules! trigger_card {
    ($name:ident) => {
        pub fn $name(&mut self, queue: &mut ActionQueue, play: &PlayCardAction) {
            if let Some(f) = self.class.$name() {
                f(&mut self.value, queue, play)
            }
        }
    };
}

impl Relic {
    trigger!(pre_combat);
    trigger!(combat_start_pre_draw);
    trigger!(combat_start_post_draw);
    trigger!(turn_start);
    trigger!(turn_end);
    trigger!(combat_finish);
    trigger_card!(on_card_played);
}

pub fn new_relic(class: RelicClass) -> Relic {
    Relic { class, value: 0 }
}

lazy_static! {
    static ref ALL_COMMON: Vec<RelicClass> = RelicClass::all()
        .into_iter()
        .filter(|r| r.rarity() == RelicRarity::Common)
        .collect();
}

pub fn random_common_relic(rng: &mut Rand) -> RelicClass {
    rand_slice(rng, &ALL_COMMON)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions::block::BlockAction,
        cards::CardClass,
        game::{GameBuilder, Move},
        status::Status,
    };

    #[test]
    fn test_burning_blood() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::DebugKill)
            .add_relic(RelicClass::BurningBlood)
            .build_combat();
        let hp = g.player.creature.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.player.creature.cur_hp, hp + 6);
    }

    #[test]
    fn test_blood_vial() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::BloodVial)
            .set_player_hp(50)
            .build_combat();
        assert_eq!(g.player.creature.cur_hp, 52);
    }

    #[test]
    fn test_bag_of_prep() {
        let g = GameBuilder::default()
            .ironclad_starting_deck()
            .add_relic(RelicClass::BagOfPrep)
            .build_combat();
        assert_eq!(g.hand.len(), 7);
    }

    #[test]
    fn test_medical_kit() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::Wound)
            .add_card(CardClass::Dazed)
            .add_relic(RelicClass::MedicalKit)
            .set_player_hp(50)
            .build_combat();
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
        assert_eq!(g.player.creature.cur_hp, 50);
        assert_eq!(g.exhaust_pile.len(), 2);
        assert_eq!(g.discard_pile.len(), 0);
    }

    #[test]
    fn test_blue_candle() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::AscendersBane)
            .add_card(CardClass::Injury)
            .add_relic(RelicClass::BlueCandle)
            .set_player_hp(50)
            .build_combat();
        g.run_action(BlockAction::player_flat_amount(5));
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: None,
        });
        assert_eq!(g.energy, 3);
        assert_eq!(g.player.creature.cur_hp, 48);
        assert_eq!(g.exhaust_pile.len(), 2);
        assert_eq!(g.discard_pile.len(), 0);
    }

    #[test]
    fn test_anchor() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Anchor)
            .build_combat();
        assert_eq!(g.player.creature.block, 10);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 0);
    }

    #[test]
    fn test_anchor_dexterity() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Anchor)
            .add_player_status(Status::Dexterity, 55)
            .build_combat();
        assert_eq!(g.player.creature.block, 10);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 0);
    }

    #[test]
    fn test_horn_cleat() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::HornCleat)
            .build_combat();
        assert_eq!(g.player.creature.block, 0);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 14);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 0);
    }

    #[test]
    fn test_captains_wheel() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::CaptainsWheel)
            .build_combat();
        assert_eq!(g.player.creature.block, 0);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 0);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 18);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.block, 0);
    }

    #[test]
    fn test_kunai() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Kunai)
            .build_combat();
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.get_status(Status::Dexterity), None);
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.get_status(Status::Dexterity), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.get_status(Status::Dexterity), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.get_status(Status::Dexterity), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.get_status(Status::Dexterity), Some(2));
    }

    #[test]
    fn test_shruiken() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Shruiken)
            .build_combat();
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::GoodInstincts, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.get_status(Status::Strength), None);
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.get_status(Status::Strength), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::GoodInstincts, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::GoodInstincts, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::GoodInstincts, Some(CreatureRef::monster(0)));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.creature.get_status(Status::Strength), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Berserk, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.get_status(Status::Strength), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.creature.get_status(Status::Strength), Some(2));
    }
}
