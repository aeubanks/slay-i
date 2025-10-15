use lazy_static::lazy_static;

use crate::{
    actions::{
        add_card_to_master_deck::AddCardToMasterDeckAction, block::BlockAction,
        choose_gamble::ChooseGambleAction, damage::DamageAction,
        damage_all_monsters::DamageAllMonstersAction,
        discount_random_card_in_hand::DiscountRandomCardInHandAction, draw::DrawAction,
        duvu::DuvuAction, enchiridion::EnchiridionAction, gain_energy::GainEnergyAction,
        gain_gold::GainGoldAction, gain_status::GainStatusAction,
        gain_status_all_monsters::GainStatusAllMonstersAction, heal::HealAction,
        increase_draw_per_turn::IncreaseDrawPerTurnAction, increase_max_hp::IncreaseMaxHPAction,
        increase_potion_slots::IncreasePotionSlotsAction, play_card::PlayCardAction,
        try_remove_card_from_master_deck::TryRemoveCardFromMasterDeckAction,
        upgrade_random_in_hand::UpgradeRandomInHandAction,
        upgrade_random_in_master::UpgradeRandomInMasterAction,
    },
    cards::{CardClass, CardType},
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

    Akabeko => Common,
    Anchor => Common,
    AncientTeaSet => Common, // TODO
    ArtOfWar => Common,
    BagOfMarbles => Common,
    BagOfPrep => Common,
    BloodVial => Common,
    BronzeScales => Common,
    CentennialPuzzle => Common,
    CeramicFish => Common,
    DreamCatcher => Common, // TODO
    HappyFlower => Common,
    JuzuBracelet => Common, // TODO
    Lantern => Common,
    MawBank => Common, // TODO
    MealTicket => Common, // TODO
    Nunchaku => Common,
    OddlySmoothStone => Common,
    Omamori => Common,
    Orichalcum => Common, // TODO
    PenNib => Common,
    PotionBelt => Common,
    PreservedInsect => Common, // TODO
    RegalPillow => Common, // TODO
    SmilingMask => Common, // TODO
    Strawberry => Common,
    Boot => Common,
    TinyChest => Common, // TODO
    ToyOrnithopter => Common,
    Vajra => Common,
    WarPaint => Common,
    Whetstone => Common,
    RedSkull => Common, // TODO

    BlueCandle => Uncommon,
    BottledFlame => Uncommon, // TODO
    BottledLightning => Uncommon, // TODO
    BottledTornado => Uncommon, // TODO
    DarkstonePeriapt => Uncommon,
    EternalFeather => Uncommon, // TODO
    FrozenEgg => Uncommon, // TODO
    GremlinHorn => Uncommon,
    HornCleat => Uncommon,
    InkBottle => Uncommon,
    Kunai => Uncommon,
    LetterOpener => Uncommon,
    Matryoshka => Uncommon, // TODO
    MeatOnTheBone => Uncommon, // TODO
    MercuryHourglass => Uncommon,
    MoltenEgg => Uncommon, // TODO
    MummifiedHand => Uncommon,
    OrnamentalFan => Uncommon,
    Pantograph => Uncommon, // TODO
    Pear => Uncommon,
    QuestionCard => Uncommon, // TODO
    Shruiken => Uncommon,
    SingingBowl => Uncommon, // TODO
    StrikeDummy => Uncommon,
    Sundial => Uncommon,
    TheCourier => Uncommon, // TODO
    ToxicEgg => Uncommon, // TODO
    WhiteBeastStatue => Uncommon, // TODO
    PaperPhrog => Uncommon,
    SelfFormingClay => Uncommon,

    BirdFacedUrn => Rare,
    Calipers => Rare,
    CaptainsWheel => Rare,
    DeadBranch => Rare, // TODO
    DuVuDoll => Rare,
    FossilizedHelix => Rare,
    GamblingChip => Rare,
    Ginger => Rare,
    Girya => Rare, // TODO
    IceCream => Rare,
    IncenseBurner => Rare,
    LizardTail => Rare,
    Mango => Rare,
    OldCoin => Rare,
    PeacePipe => Rare, // TODO
    Pocketwatch => Rare,
    PrayerWheel => Rare, // TODO
    Shovel => Rare, // TODO
    StoneCalendar => Rare,
    ThreadAndNeedle => Rare,
    Torii => Rare, // TODO
    TungstenRod => Rare,
    Turnip => Rare,
    UnceasingTop => Rare, // TODO
    WingBoots => Rare, // TODO
    ChampionBelt => Rare,
    CharonsAshes => Rare, // TODO
    MagicFlower => Rare, // TODO

    Cauldron => Shop, // TODO
    ChemicalX => Shop,
    ClockworkSouvenir => Shop,
    DollysMirror => Shop, // TODO
    FrozenEye => Shop, // TODO
    HandDrill => Shop, // TODO
    LeesWaffle => Shop,
    MedicalKit => Shop,
    MembershipCard => Shop, // TODO
    OrangePellets => Shop, // TODO
    Orrery => Shop, // TODO
    // PrismaticShard => Shop, // not supported
    SlingOfCourage => Shop, // TODO
    StrangeSpoon => Shop,
    TheAbacus => Shop,
    Toolbox => Shop, // TODO, requires pausing
    Brimstone => Shop,

    Astrolabe => Boss, // TODO
    BlackStar => Boss, // TODO
    BustedCrown => Boss, // TODO
    CallingBell => Boss, // TODO
    CoffeeDripper => Boss, // TODO
    CursedKey => Boss, // TODO
    Ectoplasm => Boss,
    EmptyCage => Boss, // TODO
    FusionHammer => Boss, // TODO
    PandorasBox => Boss, // TODO
    PhilosophersStone => Boss, // TODO
    // RunicDome => Boss, // not supported
    RunicPyramid => Boss, // TODO
    SacredBark => Boss,
    SlaversCollar => Boss, // TODO
    SneckoEye => Boss,
    Sozu => Boss, // TODO
    TinyHouse => Boss, // TODO
    VelvetChoker => Boss, // TODO
    BlackBlood => Boss,
    MarkOfPain => Boss, // TODO
    RunicCube => Boss, // TODO

    BloodyIdol => Event,
    CultistHeadpiece => Event,
    Enchiridion => Event,
    FaceOfCleric => Event,
    GoldenIdol => Event, // TODO
    GremlinVisage => Event,
    MarkOfTheBloom => Event,
    MutagenicStrength => Event,
    NlothsGift => Event, // TODO
    NlothsHungryFace => Event, // TODO
    Necronomicon => Event,
    NeowsLament => Event, // TODO
    NilryCodex => Event, // TODO, requires pausing
    OddMushroom => Event,
    RedMask => Event,
    SpiritPoop => Event,
    SsserpentHead => Event, // TODO
    WarpedTongs => Event,
);

type RelicCallback = fn(&mut i32, &mut ActionQueue);
type RelicCardCallback = fn(&mut i32, &mut ActionQueue, &mut Vec<PlayCardAction>, &PlayCardAction);

impl RelicClass {
    pub fn on_equip(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            LizardTail => Some(set_value_one),
            Omamori => Some(set_value_2),
            WarPaint => Some(war_paint),
            Whetstone => Some(whetstone),
            SneckoEye => Some(snecko_eye_equip),
            OldCoin => Some(old_coin),
            PotionBelt => Some(potion_belt),
            LeesWaffle => Some(lees_waffle),
            Mango => Some(mango),
            Pear => Some(pear),
            Strawberry => Some(strawberry),
            Necronomicon => Some(necronomicon_equip),
            _ => None,
        }
    }
    pub fn on_unequip(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            SneckoEye => Some(snecko_eye_unequip),
            Necronomicon => Some(necronomicon_unequip),
            _ => None,
        }
    }
    pub fn on_shuffle(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            Sundial => Some(sundial),
            TheAbacus => Some(abacus),
            _ => None,
        }
    }
    pub fn at_pre_combat(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            HornCleat | CaptainsWheel | ArtOfWar | StoneCalendar => Some(set_value_zero),
            CentennialPuzzle => Some(set_value_one),
            Pocketwatch => Some(set_value_99),
            SneckoEye => Some(snecko_eye_confused),
            Enchiridion => Some(enchiridion),
            _ => None,
        }
    }
    pub fn at_combat_finish(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            BurningBlood => Some(burning_blood),
            BlackBlood => Some(black_blood),
            FaceOfCleric => Some(face_of_cleric),
            _ => None,
        }
    }
    pub fn at_combat_begin_pre_draw(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            BloodVial => Some(blood_vial),
            Lantern => Some(lantern),
            GremlinVisage => Some(gremlin_visage),
            MutagenicStrength => Some(mutagenic_strength),
            ClockworkSouvenir => Some(clockwork_souvenir),
            RedMask => Some(red_mask),
            BagOfMarbles => Some(bag_of_marbles),
            BronzeScales => Some(bronze_scales),
            Vajra => Some(vajra),
            OddlySmoothStone => Some(oddly_smooth_stone),
            DuVuDoll => Some(du_vu_doll),
            Akabeko => Some(akabeko),
            PenNib => Some(pen_nib_start),
            FossilizedHelix => Some(fossilized_helix),
            ThreadAndNeedle => Some(thread_and_needle),
            _ => None,
        }
    }
    pub fn at_combat_begin_post_draw(&self) -> Option<RelicCallback> {
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
            InkBottle => Some(ink_bottle),
            LetterOpener => Some(letter_opener),
            OrnamentalFan => Some(ornamental_fan),
            Nunchaku => Some(nunchaku),
            BirdFacedUrn => Some(bird_faced_urn),
            ArtOfWar => Some(art_of_war_card_played),
            Pocketwatch => Some(pocketwatch_card_played),
            PenNib => Some(pen_nib),
            Necronomicon => Some(necronomicon),
            MummifiedHand => Some(mummified_hand),
            _ => None,
        }
    }
    pub fn at_turn_begin_pre_draw(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            Kunai | Shruiken | LetterOpener | OrnamentalFan => Some(set_value_zero),
            Necronomicon => Some(set_value_one),
            HornCleat => Some(horn_cleat),
            CaptainsWheel => Some(captains_wheel),
            HappyFlower => Some(happy_flower),
            ArtOfWar => Some(art_of_war),
            IncenseBurner => Some(incense_burner),
            MercuryHourglass => Some(mercury_hourglass),
            Brimstone => Some(brimstone),
            _ => None,
        }
    }
    pub fn at_turn_begin_post_draw(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            Pocketwatch => Some(pocketwatch),
            WarpedTongs => Some(warped_tongs),
            GamblingChip => Some(gambling_chip),
            _ => None,
        }
    }
    pub fn on_lose_hp(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            CentennialPuzzle => Some(centennial_puzzle),
            SelfFormingClay => Some(self_forming_clay),
            _ => None,
        }
    }
    pub fn at_turn_end(&self) -> Option<RelicCallback> {
        use RelicClass::*;
        match self {
            StoneCalendar => Some(stone_calendar),
            _ => None,
        }
    }
}

fn set_value_zero(v: &mut i32, _: &mut ActionQueue) {
    *v = 0;
}

fn set_value_one(v: &mut i32, _: &mut ActionQueue) {
    *v = 1;
}

fn set_value_2(v: &mut i32, _: &mut ActionQueue) {
    *v = 2;
}

fn set_value_99(v: &mut i32, _: &mut ActionQueue) {
    *v = 99;
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

fn pocketwatch_card_played(
    v: &mut i32,
    _: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    _: &PlayCardAction,
) {
    *v += 1;
}

fn pocketwatch(v: &mut i32, queue: &mut ActionQueue) {
    if *v <= 3 {
        queue.push_bot(DrawAction(3));
    }
    *v = 0;
}

fn warped_tongs(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(UpgradeRandomInHandAction());
}

fn gambling_chip(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(ChooseGambleAction());
}

fn centennial_puzzle(v: &mut i32, queue: &mut ActionQueue) {
    if *v == 1 {
        *v = 0;
        // push_top is intentional
        queue.push_top(DrawAction(3));
    }
}

fn self_forming_clay(_: &mut i32, queue: &mut ActionQueue) {
    // push_top is intentional
    queue.push_top(GainStatusAction {
        status: Status::NextTurnBlock,
        amount: 3,
        target: CreatureRef::player(),
    });
}

fn pen_nib_start(v: &mut i32, queue: &mut ActionQueue) {
    if *v == 9 {
        queue.push_bot(GainStatusAction {
            status: Status::PenNib,
            amount: 1,
            target: CreatureRef::player(),
        });
    }
}

fn necronomicon_equip(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(AddCardToMasterDeckAction(CardClass::Necronomicurse));
}

fn necronomicon_unequip(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(TryRemoveCardFromMasterDeckAction(CardClass::Necronomicurse));
}

fn necronomicon(
    v: &mut i32,
    _: &mut ActionQueue,
    card_queue: &mut Vec<PlayCardAction>,
    play: &PlayCardAction,
) {
    if *v == 1 && play.card.borrow().class.ty() == CardType::Attack && play.cost >= 2 {
        *v = 0;
        card_queue.push(PlayCardAction::duplicated(play));
    }
}

fn pen_nib(
    v: &mut i32,
    queue: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    play: &PlayCardAction,
) {
    if play.card.borrow().class.ty() == CardType::Attack {
        inc_wrap(v, 10);
        if *v == 9 {
            queue.push_bot(GainStatusAction {
                status: Status::PenNib,
                amount: 1,
                target: CreatureRef::player(),
            });
        }
    }
}

fn kunai(
    v: &mut i32,
    queue: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    play_card: &PlayCardAction,
) {
    if play_card.card.borrow().class.ty() == CardType::Attack && inc_wrap(v, 3) {
        queue.push_bot(GainStatusAction {
            status: Status::Dexterity,
            amount: 1,
            target: CreatureRef::player(),
        });
    }
}

fn shruiken(
    v: &mut i32,
    queue: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    play_card: &PlayCardAction,
) {
    if play_card.card.borrow().class.ty() == CardType::Attack && inc_wrap(v, 3) {
        queue.push_bot(GainStatusAction {
            status: Status::Strength,
            amount: 1,
            target: CreatureRef::player(),
        });
    }
}

fn ink_bottle(
    v: &mut i32,
    queue: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    _: &PlayCardAction,
) {
    if inc_wrap(v, 10) {
        queue.push_bot(DrawAction(1));
    }
}

fn mummified_hand(
    _: &mut i32,
    queue: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    play: &PlayCardAction,
) {
    if play.card.borrow().class.ty() == CardType::Power {
        queue.push_bot(DiscountRandomCardInHandAction());
    }
}

fn ornamental_fan(
    v: &mut i32,
    queue: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    play: &PlayCardAction,
) {
    if play.card.borrow().class.ty() == CardType::Attack && inc_wrap(v, 3) {
        queue.push_bot(BlockAction::player_flat_amount(4));
    }
}

fn nunchaku(
    v: &mut i32,
    queue: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    play: &PlayCardAction,
) {
    if play.card.borrow().class.ty() == CardType::Attack && inc_wrap(v, 10) {
        queue.push_bot(GainEnergyAction(1));
    }
}

fn bird_faced_urn(
    _: &mut i32,
    queue: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    play: &PlayCardAction,
) {
    if play.card.borrow().class.ty() == CardType::Power {
        queue.push_bot(HealAction {
            target: CreatureRef::player(),
            amount: 2,
        });
    }
}

fn letter_opener(
    v: &mut i32,
    queue: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    play: &PlayCardAction,
) {
    if play.card.borrow().class.ty() == CardType::Skill && inc_wrap(v, 3) {
        queue.push_bot(DamageAllMonstersAction::thorns(5));
    }
}

fn burning_blood(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(HealAction {
        target: CreatureRef::player(),
        amount: 6,
    });
}
fn black_blood(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(HealAction {
        target: CreatureRef::player(),
        amount: 12,
    });
}

fn face_of_cleric(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(IncreaseMaxHPAction(1));
}

fn blood_vial(_: &mut i32, queue: &mut ActionQueue) {
    // push_top is intentional
    queue.push_top(HealAction {
        target: CreatureRef::player(),
        amount: 2,
    });
}

fn lantern(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(GainEnergyAction(1));
}

fn red_mask(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(GainStatusAllMonstersAction {
        status: Status::Weak,
        amount: 1,
    });
}

fn bag_of_marbles(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(GainStatusAllMonstersAction {
        status: Status::Vulnerable,
        amount: 1,
    });
}

fn akabeko(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(GainStatusAction {
        status: Status::Vigor,
        amount: 8,
        target: CreatureRef::player(),
    });
}

fn bronze_scales(_: &mut i32, queue: &mut ActionQueue) {
    // push_top is intentional
    queue.push_top(GainStatusAction {
        status: Status::Thorns,
        amount: 3,
        target: CreatureRef::player(),
    });
}

fn du_vu_doll(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(DuvuAction());
}

fn fossilized_helix(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(GainStatusAction {
        status: Status::Buffer,
        amount: 1,
        target: CreatureRef::player(),
    });
}

fn thread_and_needle(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(GainStatusAction {
        status: Status::PlatedArmor,
        amount: 4,
        target: CreatureRef::player(),
    });
}

fn vajra(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(GainStatusAction {
        status: Status::Strength,
        amount: 1,
        target: CreatureRef::player(),
    });
}

fn oddly_smooth_stone(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(GainStatusAction {
        status: Status::Dexterity,
        amount: 1,
        target: CreatureRef::player(),
    });
}

fn gremlin_visage(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(GainStatusAction {
        status: Status::Weak,
        amount: 1,
        target: CreatureRef::player(),
    });
}

fn clockwork_souvenir(_: &mut i32, queue: &mut ActionQueue) {
    // push_top is intentional
    queue.push_top(GainStatusAction {
        status: Status::Artifact,
        amount: 1,
        target: CreatureRef::player(),
    });
}

fn mutagenic_strength(_: &mut i32, queue: &mut ActionQueue) {
    // push_top is intentional
    queue.push_top(GainStatusAction {
        status: Status::LoseStrength,
        amount: 3,
        target: CreatureRef::player(),
    });
    queue.push_top(GainStatusAction {
        status: Status::Strength,
        amount: 3,
        target: CreatureRef::player(),
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

fn stone_calendar(v: &mut i32, queue: &mut ActionQueue) {
    *v += 1;
    if *v == 7 {
        queue.push_bot(DamageAllMonstersAction::thorns(52));
    }
}

fn happy_flower(v: &mut i32, queue: &mut ActionQueue) {
    if inc_wrap(v, 3) {
        queue.push_bot(GainEnergyAction(1));
    }
}

fn incense_burner(v: &mut i32, queue: &mut ActionQueue) {
    if inc_wrap(v, 6) {
        queue.push_bot(GainStatusAction {
            status: Status::Intangible,
            amount: 1,
            target: CreatureRef::player(),
        });
    }
}

fn mercury_hourglass(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(DamageAllMonstersAction::thorns(3));
}

fn brimstone(_: &mut i32, queue: &mut ActionQueue) {
    // intentional push_top
    queue.push_top(GainStatusAction {
        status: Status::Strength,
        amount: 2,
        target: CreatureRef::player(),
    });
    queue.push_top(GainStatusAllMonstersAction {
        status: Status::Strength,
        amount: 1,
    });
}

fn art_of_war_card_played(
    v: &mut i32,
    _: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    play: &PlayCardAction,
) {
    if play.card.borrow().class.ty() == CardType::Attack {
        *v = 0;
    }
}

fn art_of_war(v: &mut i32, queue: &mut ActionQueue) {
    if *v == 1 {
        queue.push_bot(GainEnergyAction(1));
    }
    *v = 1;
}

fn blue_candle(
    _: &mut i32,
    queue: &mut ActionQueue,
    _: &mut Vec<PlayCardAction>,
    play: &PlayCardAction,
) {
    if play.card.borrow().class.ty() == CardType::Curse {
        queue.push_bot(DamageAction::lose_hp(1, CreatureRef::player()));
    }
}

fn abacus(_: &mut i32, queue: &mut ActionQueue) {
    // intentionally push_top, see ShuffleDiscardOnTopOfDrawAction
    queue.push_top(BlockAction::player_flat_amount(6));
}

fn sundial(v: &mut i32, queue: &mut ActionQueue) {
    if inc_wrap(v, 3) {
        // intentionally push_top, see ShuffleDiscardOnTopOfDrawAction
        queue.push_top(GainEnergyAction(2));
    }
}

fn potion_belt(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(IncreasePotionSlotsAction(2));
}

fn old_coin(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(GainGoldAction(300));
}

fn lees_waffle(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(IncreaseMaxHPAction(7));
    queue.push_bot(HealAction {
        target: CreatureRef::player(),
        amount: 9999,
    });
}

fn mango(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(IncreaseMaxHPAction(14));
}

fn pear(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(IncreaseMaxHPAction(10));
}

fn strawberry(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(IncreaseMaxHPAction(7));
}

fn enchiridion(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(EnchiridionAction());
}

fn snecko_eye_confused(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(GainStatusAction {
        status: Status::Confusion,
        amount: 1,
        target: CreatureRef::player(),
    });
}

fn snecko_eye_equip(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(IncreaseDrawPerTurnAction(2));
}

fn snecko_eye_unequip(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(IncreaseDrawPerTurnAction(-2));
}

fn war_paint(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(UpgradeRandomInMasterAction(CardType::Skill));
}

fn whetstone(_: &mut i32, queue: &mut ActionQueue) {
    queue.push_bot(UpgradeRandomInMasterAction(CardType::Attack));
}

pub struct Relic {
    class: RelicClass,
    value: i32,
}

impl Relic {
    pub fn get_class(&self) -> RelicClass {
        self.class
    }
    pub fn get_value(&self) -> i32 {
        self.value
    }
    pub fn set_value(&mut self, v: i32) {
        self.value = v;
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
        pub fn $name(
            &mut self,
            queue: &mut ActionQueue,
            card_queue: &mut Vec<PlayCardAction>,
            play: &PlayCardAction,
        ) {
            if let Some(f) = self.class.$name() {
                f(&mut self.value, queue, card_queue, play)
            }
        }
    };
}

impl Relic {
    trigger!(on_equip);
    trigger!(on_unequip);
    trigger!(on_shuffle);
    trigger!(at_pre_combat);
    trigger!(at_combat_begin_pre_draw);
    trigger!(at_combat_begin_post_draw);
    trigger!(at_turn_begin_pre_draw);
    trigger!(at_turn_begin_post_draw);
    trigger!(at_turn_end);
    trigger!(on_lose_hp);
    trigger!(at_combat_finish);
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
        actions::{add_card_to_master_deck::AddCardToMasterDeckAction, block::BlockAction},
        assert_matches,
        cards::{CardClass, CardColor},
        game::{GameBuilder, GameStatus, Move},
        monster::Monster,
        monsters::test::{ApplyStatusMonster, AttackMonster, NoopMonster},
        potion::Potion,
        status::Status,
    };

    #[test]
    fn test_burning_blood() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::DebugKill)
            .add_relic(RelicClass::BurningBlood)
            .build_combat();
        let hp = g.player.cur_hp;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.player.cur_hp, hp + 6);
    }

    #[test]
    fn test_black_blood() {
        let mut g = GameBuilder::default()
            .add_card(CardClass::DebugKill)
            .add_relic(RelicClass::BlackBlood)
            .build_combat();
        g.player.cur_hp = 10;
        g.make_move(Move::PlayCard {
            card_index: 0,
            target: Some(0),
        });
        assert_eq!(g.player.cur_hp, 10 + 12);
    }

    #[test]
    fn test_face_of_cleric() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::FaceOfCleric)
            .build_combat();
        let hp = g.player.max_hp;
        g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.max_hp, hp + 1);
    }

    #[test]
    fn test_blood_vial() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::BloodVial)
            .set_player_hp(50)
            .build_combat();
        assert_eq!(g.player.cur_hp, 52);
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
        assert_eq!(g.player.cur_hp, 50);
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
        assert_eq!(g.player.cur_hp, 48);
        assert_eq!(g.exhaust_pile.len(), 2);
        assert_eq!(g.discard_pile.len(), 0);
    }

    #[test]
    fn test_anchor() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Anchor)
            .build_combat();
        assert_eq!(g.player.block, 10);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 0);
    }

    #[test]
    fn test_anchor_dexterity() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Anchor)
            .add_player_status(Status::Dexterity, 55)
            .build_combat();
        assert_eq!(g.player.block, 10);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 0);
    }

    #[test]
    fn test_horn_cleat() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::HornCleat)
            .build_combat();
        assert_eq!(g.player.block, 0);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 14);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 0);
    }

    #[test]
    fn test_captains_wheel() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::CaptainsWheel)
            .build_combat();
        assert_eq!(g.player.block, 0);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 0);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 18);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 0);
    }

    #[test]
    fn test_kunai() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Kunai)
            .build_combat();
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.get_status(Status::Dexterity), None);
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.get_status(Status::Dexterity), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.get_status(Status::Dexterity), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.get_status(Status::Dexterity), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.get_status(Status::Dexterity), Some(2));
    }

    #[test]
    fn test_shruiken() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Shruiken)
            .build_combat();
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::GoodInstincts, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.get_status(Status::Strength), None);
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.get_status(Status::Strength), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::GoodInstincts, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::GoodInstincts, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::GoodInstincts, Some(CreatureRef::monster(0)));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.get_status(Status::Strength), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Berserk, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.get_status(Status::Strength), Some(1));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.get_status(Status::Strength), Some(2));
    }

    #[test]
    fn test_ink_bottle() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::InkBottle)
            .build_combat();
        g.add_card_to_draw_pile(CardClass::Strike);
        for i in 0..9 {
            g.play_card(CardClass::Bloodletting, None);
            assert_eq!(g.get_relic_value(RelicClass::InkBottle), Some(i + 1));
            assert_eq!(g.hand.len(), 0);
        }
        g.play_card(CardClass::Bloodletting, None);
        assert_eq!(g.get_relic_value(RelicClass::InkBottle), Some(0));
        assert_eq!(g.hand.len(), 1);
        g.play_card(CardClass::Bloodletting, None);
        assert_eq!(g.get_relic_value(RelicClass::InkBottle), Some(1));
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::InkBottle), Some(1));
    }

    #[test]
    fn test_strike_dummy() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::StrikeDummy)
            .build_combat();
        g.energy = 99;
        let hp = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::Strike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 9);
        g.play_card(CardClass::TwinStrike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - (6 + 3) - (5 + 3) * 2);
        g.play_card(CardClass::Bash, Some(CreatureRef::monster(0)));
        assert_eq!(
            g.monsters[0].creature.cur_hp,
            hp - (6 + 3) - (5 + 3) * 2 - 8
        );
    }

    #[test]
    fn test_letter_opener() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::LetterOpener)
            .add_monster(NoopMonster::with_hp(50))
            .add_monster(NoopMonster::with_hp(50))
            .build_combat();
        g.energy = 99;
        g.play_card(CardClass::Strike, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Defend, None);
        g.play_card(CardClass::Defend, None);
        assert_eq!(g.get_relic_value(RelicClass::LetterOpener), Some(2));
        assert_eq!(g.monsters[0].creature.cur_hp, 50 - 6);
        assert_eq!(g.monsters[1].creature.cur_hp, 50);
        g.play_card(CardClass::Defend, None);
        assert_eq!(g.get_relic_value(RelicClass::LetterOpener), Some(0));
        assert_eq!(g.monsters[0].creature.cur_hp, 50 - 6 - 5);
        assert_eq!(g.monsters[1].creature.cur_hp, 50 - 5);
        g.play_card(CardClass::Defend, None);
        assert_eq!(g.get_relic_value(RelicClass::LetterOpener), Some(1));
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::LetterOpener), Some(0));
    }

    #[test]
    fn test_ornamental_fan() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::OrnamentalFan)
            .build_combat();
        assert_eq!(g.get_relic_value(RelicClass::OrnamentalFan), Some(0));
        g.play_card(CardClass::Intimidate, None);
        g.play_card(CardClass::Whirlwind, None);
        assert_eq!(g.get_relic_value(RelicClass::OrnamentalFan), Some(1));
        g.play_card(CardClass::Whirlwind, None);
        assert_eq!(g.get_relic_value(RelicClass::OrnamentalFan), Some(2));
        assert_eq!(g.player.block, 0);
        g.play_card(CardClass::Whirlwind, None);
        assert_eq!(g.get_relic_value(RelicClass::OrnamentalFan), Some(0));
        assert_eq!(g.player.block, 4);
        g.play_card(CardClass::Intimidate, None);
        assert_eq!(g.get_relic_value(RelicClass::OrnamentalFan), Some(0));
        g.play_card(CardClass::Whirlwind, None);
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::OrnamentalFan), Some(0));
    }

    #[test]
    fn test_abacus() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::TheAbacus)
            .build_combat();
        assert_eq!(g.player.block, 0);
        g.play_card(CardClass::Thunderclap, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.block, 0);
        g.play_card(CardClass::PommelStrike, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.block, 6);
        g.play_card(CardClass::PommelStrike, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.block, 12);
        g.play_card(CardClass::MasterOfStrategy, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.block, 24);
        g.play_card(CardClass::MasterOfStrategy, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.block, 24);
    }

    #[test]
    fn test_sundial() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Sundial)
            .build_combat();

        g.add_cards_to_discard_pile(CardClass::Strike, 10);
        assert_eq!(g.get_relic_value(RelicClass::Sundial), Some(0));
        g.play_card(CardClass::FlashOfSteel, Some(CreatureRef::monster(0)));
        assert_eq!(g.get_relic_value(RelicClass::Sundial), Some(1));
        assert_eq!(g.energy, 3);

        g.clear_all_piles();
        g.add_cards_to_discard_pile(CardClass::Strike, 10);
        g.play_card(CardClass::FlashOfSteel, Some(CreatureRef::monster(0)));
        assert_eq!(g.get_relic_value(RelicClass::Sundial), Some(2));
        assert_eq!(g.energy, 3);

        g.clear_all_piles();
        g.add_cards_to_discard_pile(CardClass::Strike, 10);
        g.play_card(CardClass::FlashOfSteel, Some(CreatureRef::monster(0)));
        assert_eq!(g.get_relic_value(RelicClass::Sundial), Some(0));
        assert_eq!(g.energy, 5);

        g.clear_all_piles();
        g.add_cards_to_discard_pile(CardClass::Strike, 10);
        g.play_card(CardClass::FlashOfSteel, Some(CreatureRef::monster(0)));
        assert_eq!(g.get_relic_value(RelicClass::Sundial), Some(1));
        assert_eq!(g.energy, 5);

        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::Sundial), Some(1));
    }

    #[test]
    fn test_sundial_infinite() {
        {
            let mut g = GameBuilder::default()
                .add_relic(RelicClass::Sundial)
                .add_monster(NoopMonster::with_hp(10000))
                .add_cards_upgraded(CardClass::PommelStrike, 2)
                .build_combat();
            for _ in 0..100 {
                g.make_move(Move::PlayCard {
                    card_index: 0,
                    target: Some(0),
                });
            }
            assert!(g.energy > 10);
        }
        {
            let mut g = GameBuilder::default()
                .add_relic(RelicClass::Sundial)
                .add_monster(NoopMonster::with_hp(10000))
                .add_card(CardClass::PommelStrike)
                .add_card_upgraded(CardClass::PommelStrike)
                .build_combat();
            for _ in 0..100 {
                g.make_move(Move::PlayCard {
                    card_index: 0,
                    target: Some(0),
                });
            }
            assert!(g.energy < 10);
        }
    }

    #[test]
    fn test_combat_begin() {
        {
            let g = GameBuilder::default()
                .add_relic(RelicClass::MutagenicStrength)
                .add_relic(RelicClass::ClockworkSouvenir)
                .add_relic(RelicClass::GremlinVisage)
                .build_combat();
            assert_eq!(g.player.get_status(Status::Weak), Some(1));
            assert_eq!(g.player.get_status(Status::Artifact), None);
            assert_eq!(g.player.get_status(Status::Strength), Some(3));
            assert_eq!(g.player.get_status(Status::LoseStrength), None);
        }
        {
            let g = GameBuilder::default()
                .add_relic(RelicClass::ClockworkSouvenir)
                .add_relic(RelicClass::MutagenicStrength)
                .add_relic(RelicClass::GremlinVisage)
                .build_combat();
            assert_eq!(g.player.get_status(Status::Weak), None);
            assert_eq!(g.player.get_status(Status::Artifact), None);
            assert_eq!(g.player.get_status(Status::Strength), Some(3));
            assert_eq!(g.player.get_status(Status::LoseStrength), Some(3));
        }
        {
            let g = GameBuilder::default()
                .add_relic(RelicClass::GremlinVisage)
                .add_relic(RelicClass::ClockworkSouvenir)
                .add_relic(RelicClass::MutagenicStrength)
                .build_combat();
            assert_eq!(g.player.get_status(Status::Weak), None);
            assert_eq!(g.player.get_status(Status::Artifact), None);
            assert_eq!(g.player.get_status(Status::Strength), Some(3));
            assert_eq!(g.player.get_status(Status::LoseStrength), Some(3));
        }
    }

    #[test]
    fn test_snecko_eye() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::ClockworkSouvenir)
            .add_relic(RelicClass::SneckoEye)
            .add_cards(CardClass::Strike, 10)
            .build_combat();
        assert_eq!(g.player.get_status(Status::Confusion), Some(1));
        assert_eq!(g.hand.len(), 7);
        g.remove_relic(RelicClass::SneckoEye);
        g.make_move(Move::EndTurn);
        assert_eq!(g.hand.len(), 5);
    }

    #[test]
    fn test_lantern() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Lantern)
            .build_combat();
        assert_eq!(g.energy, 4);
        g.make_move(Move::EndTurn);
        assert_eq!(g.energy, 3);
    }

    #[test]
    fn test_happy_flower() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::HappyFlower)
            .build_combat();
        assert_eq!(g.get_relic_value(RelicClass::HappyFlower), Some(1));
        assert_eq!(g.energy, 3);
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::HappyFlower), Some(2));
        assert_eq!(g.energy, 3);
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::HappyFlower), Some(0));
        assert_eq!(g.energy, 4);
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::HappyFlower), Some(1));
        assert_eq!(g.energy, 3);
    }

    #[test]
    fn test_incense_burner() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::IncenseBurner)
            .build_combat();

        g.combat_monsters_queue
            .push(vec![Monster::new(NoopMonster::new(), &mut g.rng)]);

        assert_eq!(g.get_relic_value(RelicClass::IncenseBurner), Some(1));
        assert_eq!(g.player.get_status(Status::Intangible), None);
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::IncenseBurner), Some(2));
        assert_eq!(g.player.get_status(Status::Intangible), None);
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::IncenseBurner), Some(3));
        assert_eq!(g.player.get_status(Status::Intangible), None);
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::IncenseBurner), Some(4));
        assert_eq!(g.player.get_status(Status::Intangible), None);
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::IncenseBurner), Some(5));
        assert_eq!(g.player.get_status(Status::Intangible), None);
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::IncenseBurner), Some(0));
        assert_eq!(g.player.get_status(Status::Intangible), Some(1));
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::IncenseBurner), Some(1));
        assert_eq!(g.player.get_status(Status::Intangible), None);
        g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(0)));
        assert_eq!(g.get_relic_value(RelicClass::IncenseBurner), Some(2));
    }

    #[test]
    fn test_ectoplasm() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Ectoplasm)
            .add_monster(NoopMonster::with_hp(1))
            .build_combat();
        assert_eq!(g.energy, 4);
        g.play_card(CardClass::HandOfGreed, Some(CreatureRef::monster(0)));
        assert_eq!(g.gold, 0);
    }

    #[test]
    fn test_food() {
        let mut g = GameBuilder::default().build_combat();
        let max_hp = g.player.max_hp;
        g.add_relic(RelicClass::Mango);
        assert_eq!(g.player.max_hp, max_hp + 14);
        g.add_relic(RelicClass::Pear);
        assert_eq!(g.player.max_hp, max_hp + 14 + 10);
        g.add_relic(RelicClass::Strawberry);
        assert_eq!(g.player.max_hp, max_hp + 14 + 10 + 7);
        assert_ne!(g.player.max_hp, g.player.cur_hp);
        g.add_relic(RelicClass::LeesWaffle);
        assert_eq!(g.player.max_hp, max_hp + 14 + 10 + 7 + 7);
        assert_eq!(g.player.max_hp, g.player.cur_hp);
    }

    #[test]
    fn test_potion_belt() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::PotionBelt)
            .build_combat();
        assert_eq!(g.potions, vec![None; 4]);
        g.remove_relic(RelicClass::PotionBelt);
        assert_eq!(g.potions, vec![None; 4]);
        for _ in 0..4 {
            g.add_potion(Potion::Ancient);
        }
    }

    #[test]
    fn test_war_paint() {
        {
            let mut g = GameBuilder::default().build_combat();
            g.add_relic(RelicClass::WarPaint);
        }
        {
            let mut g = GameBuilder::default()
                .add_card(CardClass::Strike)
                .add_card(CardClass::Defend)
                .add_card(CardClass::ShrugItOff)
                .add_card_upgraded(CardClass::BandageUp)
                .build_combat();
            g.add_relic(RelicClass::WarPaint);
            assert_eq!(
                g.master_deck
                    .iter()
                    .map(|c| c.borrow().upgrade_count)
                    .sum::<i32>(),
                3
            );
            assert!(g.master_deck.iter().any(
                |c| c.borrow().class == CardClass::Strike && c.borrow().upgrade_count == 0
            ));
        }
        {
            let mut found_unupgraded_defend = false;
            let mut found_unupgraded_shrug = false;
            let mut found_unupgraded_bandage = false;
            for _ in 0..100 {
                let mut g = GameBuilder::default()
                    .add_card(CardClass::Strike)
                    .add_card(CardClass::Defend)
                    .add_card(CardClass::ShrugItOff)
                    .add_card(CardClass::BandageUp)
                    .build_combat();
                g.add_relic(RelicClass::WarPaint);
                assert_eq!(
                    g.master_deck
                        .iter()
                        .map(|c| c.borrow().upgrade_count)
                        .sum::<i32>(),
                    2
                );
                assert!(g.master_deck.iter().any(
                    |c| c.borrow().class == CardClass::Strike && c.borrow().upgrade_count == 0
                ));
                for c in &g.master_deck {
                    if c.borrow().upgrade_count != 0 {
                        continue;
                    }
                    match c.borrow().class {
                        CardClass::Defend => found_unupgraded_defend = true,
                        CardClass::ShrugItOff => found_unupgraded_shrug = true,
                        CardClass::BandageUp => found_unupgraded_bandage = true,
                        _ => {}
                    }
                }
                if found_unupgraded_defend && found_unupgraded_shrug && found_unupgraded_bandage {
                    break;
                }
            }
            assert!(found_unupgraded_defend && found_unupgraded_shrug && found_unupgraded_bandage);
        }
    }

    #[test]
    fn test_whetstone() {
        {
            let mut g = GameBuilder::default().build_combat();
            g.add_relic(RelicClass::Whetstone);
        }
        {
            let mut g = GameBuilder::default()
                .add_card(CardClass::Bash)
                .build_combat();
            g.add_relic(RelicClass::Whetstone);
            assert_eq!(g.master_deck[0].borrow().upgrade_count, 1);
        }
        {
            let mut g = GameBuilder::default()
                .add_card(CardClass::Defend)
                .add_card(CardClass::Strike)
                .add_card_upgraded(CardClass::SearingBlow)
                .build_combat();
            g.add_relic(RelicClass::Whetstone);
            for c in &g.master_deck {
                let c = c.borrow();
                match c.class {
                    CardClass::Defend => assert_eq!(c.upgrade_count, 0),
                    CardClass::Strike => assert_eq!(c.upgrade_count, 1),
                    CardClass::SearingBlow => assert_eq!(c.upgrade_count, 2),
                    _ => panic!(),
                }
            }
        }
    }

    #[test]
    fn test_red_mask() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::RedMask)
            .build_combat();
        assert_eq!(g.monsters[0].creature.get_status(Status::Weak), Some(1));
    }

    #[test]
    fn test_bag_of_marbles() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::BagOfMarbles)
            .build_combat();
        assert_eq!(
            g.monsters[0].creature.get_status(Status::Vulnerable),
            Some(1)
        );
    }

    #[test]
    fn test_duvu_doll() {
        assert_eq!(
            GameBuilder::default()
                .add_relic(RelicClass::DuVuDoll)
                .build_combat()
                .player
                .get_status(Status::Strength),
            None
        );
        assert_eq!(
            GameBuilder::default()
                .add_card(CardClass::Strike)
                .add_relic(RelicClass::DuVuDoll)
                .build_combat()
                .player
                .get_status(Status::Strength),
            None
        );
        assert_eq!(
            GameBuilder::default()
                .add_card(CardClass::AscendersBane)
                .add_relic(RelicClass::DuVuDoll)
                .build_combat()
                .player
                .get_status(Status::Strength),
            Some(1)
        );
        assert_eq!(
            GameBuilder::default()
                .add_card(CardClass::AscendersBane)
                .add_card(CardClass::CurseOfTheBell)
                .add_relic(RelicClass::DuVuDoll)
                .build_combat()
                .player
                .get_status(Status::Strength),
            Some(2)
        );
    }

    #[test]
    fn test_vajra() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::Vajra)
            .build_combat();
        assert_eq!(g.player.get_status(Status::Strength), Some(1));
    }

    #[test]
    fn test_oddly_smooth_stone() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::OddlySmoothStone)
            .build_combat();
        assert_eq!(g.player.get_status(Status::Dexterity), Some(1));
    }

    #[test]
    fn test_odd_mushroom() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::OddMushroom)
            .add_player_status(Status::Vulnerable, 5)
            .add_monster(AttackMonster::new(10))
            .build_combat();
        let hp = g.player.cur_hp;
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.cur_hp, hp - 12);
    }

    #[test]
    fn test_paper_phrog() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::PaperPhrog)
            .add_monster_status(Status::Vulnerable, 5)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.play_card_upgraded(CardClass::Bash, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 17);
    }

    #[test]
    fn test_nunchaku() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Nunchaku)
            .build_combat();
        for i in 1..15 {
            g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
            assert_eq!(g.get_relic_value(RelicClass::Nunchaku), Some(i % 10));
            if i < 10 {
                assert_eq!(g.energy, 3);
            } else {
                assert_eq!(g.energy, 4);
            }
        }
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_relic_value(RelicClass::Nunchaku), Some(4));
    }

    #[test]
    fn test_bird_faced_urn() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::BirdFacedUrn)
            .build_combat();
        let hp = g.player.cur_hp;
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.cur_hp, hp);
        g.play_card(CardClass::Inflame, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.cur_hp, hp + 2);
    }

    #[test]
    fn test_calipers() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Calipers)
            .build_combat();
        g.player.block = 20;
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 5);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 0);
    }

    #[test]
    fn test_ice_cream() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::IceCream)
            .build_combat();
        assert_eq!(g.energy, 3);
        g.play_card(CardClass::Defend, None);
        assert_eq!(g.energy, 2);
        g.make_move(Move::EndTurn);
        assert_eq!(g.energy, 5);
        g.add_relic(RelicClass::Ectoplasm);
        g.make_move(Move::EndTurn);
        assert_eq!(g.energy, 9);
    }

    #[test]
    fn test_akabeko() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Akabeko)
            .build_combat();
        assert_eq!(g.player.get_status(Status::Vigor), Some(8));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.get_status(Status::Vigor), Some(8));
    }

    #[test]
    fn test_art_of_war() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::ArtOfWar)
            .build_combat();
        assert_eq!(g.energy, 3);
        assert_eq!(g.get_relic_value(RelicClass::ArtOfWar), Some(1));
        g.make_move(Move::EndTurn);
        assert_eq!(g.energy, 4);
        assert_eq!(g.get_relic_value(RelicClass::ArtOfWar), Some(1));
        g.play_card(CardClass::Inflame, None);
        g.play_card(CardClass::Defend, None);
        assert_eq!(g.get_relic_value(RelicClass::ArtOfWar), Some(1));
        g.make_move(Move::EndTurn);
        assert_eq!(g.energy, 4);
        assert_eq!(g.get_relic_value(RelicClass::ArtOfWar), Some(1));
        g.play_card(CardClass::Thunderclap, None);
        assert_eq!(g.get_relic_value(RelicClass::ArtOfWar), Some(0));
        g.make_move(Move::EndTurn);
        assert_eq!(g.energy, 3);
    }

    #[test]
    fn test_pocketwatch() {
        let mut g = GameBuilder::default()
            .add_cards(CardClass::Strike, 10)
            .add_relic(RelicClass::Pocketwatch)
            .build_combat();

        assert_eq!(g.hand.len(), 5);
        assert_eq!(g.get_relic_value(RelicClass::Pocketwatch), Some(0));

        g.make_move(Move::EndTurn);
        assert_eq!(g.hand.len(), 8);

        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.get_relic_value(RelicClass::Pocketwatch), Some(3));
        g.make_move(Move::EndTurn);
        assert_eq!(g.hand.len(), 8);

        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
        assert_eq!(g.get_relic_value(RelicClass::Pocketwatch), Some(4));
        g.make_move(Move::EndTurn);
        assert_eq!(g.hand.len(), 5);
        assert_eq!(g.get_relic_value(RelicClass::Pocketwatch), Some(0));
    }

    #[test]
    fn test_bronze_scales() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::BronzeScales)
            .build_combat();
        assert_eq!(g.player.get_status(Status::Thorns), Some(3));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.get_status(Status::Thorns), Some(3));
    }

    #[test]
    fn test_omamori() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Omamori)
            .build_combat();
        assert_eq!(g.get_relic_value(RelicClass::Omamori), Some(2));

        g.add_card_to_master_deck(CardClass::Anger);
        assert_eq!(g.master_deck.len(), 1);
        assert_eq!(g.get_relic_value(RelicClass::Omamori), Some(2));

        g.add_card_to_master_deck(CardClass::CurseOfTheBell);
        assert_eq!(g.master_deck.len(), 1);
        assert_eq!(g.get_relic_value(RelicClass::Omamori), Some(1));

        g.add_card_to_master_deck(CardClass::Parasite);
        assert_eq!(g.master_deck.len(), 1);
        assert_eq!(g.get_relic_value(RelicClass::Omamori), Some(0));

        g.add_card_to_master_deck(CardClass::Parasite);
        assert_eq!(g.master_deck.len(), 2);
        assert_eq!(g.get_relic_value(RelicClass::Omamori), Some(0));
    }

    #[test]
    fn test_darkstone_periapt() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::DarkstonePeriapt)
            .build_combat();

        let max_hp = g.player.max_hp;

        g.add_card_to_master_deck(CardClass::Anger);
        assert_eq!(g.player.max_hp, max_hp);

        g.add_card_to_master_deck(CardClass::CurseOfTheBell);
        assert_eq!(g.player.max_hp, max_hp + 6);

        g.add_card_to_master_deck(CardClass::AscendersBane);
        assert_eq!(g.player.max_hp, max_hp + 12);

        g.add_relic(RelicClass::Omamori);
        g.add_card_to_master_deck(CardClass::AscendersBane);
        assert_eq!(g.player.max_hp, max_hp + 12);
    }

    #[test]
    fn test_gremlin_horn() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::GremlinHorn)
            .add_monster(NoopMonster::new())
            .add_monster(AttackMonster::new(1))
            .add_monster(NoopMonster::new())
            .add_player_status(Status::Thorns, 999)
            .build_combat();
        g.add_cards_to_draw_pile(CardClass::Strike, 10);

        assert_eq!(g.energy, 3);
        assert_eq!(g.hand.len(), 0);

        g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(0)));
        assert_eq!(g.energy, 4);
        assert_eq!(g.hand.len(), 1);

        g.make_move(Move::EndTurn);
        assert_eq!(g.energy, 3);
        assert_eq!(g.hand.len(), 6);
    }

    #[test]
    fn test_pen_nib() {
        {
            let mut g = GameBuilder::default()
                .add_relic(RelicClass::PenNib)
                .build_combat();

            for i in 1..15 {
                g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
                g.play_card(CardClass::GoodInstincts, None);
                assert_eq!(g.get_relic_value(RelicClass::PenNib), Some(i % 10));
                if i == 9 {
                    assert_eq!(g.player.get_status(Status::PenNib), Some(1));
                } else {
                    assert_eq!(g.player.get_status(Status::PenNib), None);
                }
            }
        }
        {
            let mut g = GameBuilder::default()
                .add_relic(RelicClass::PenNib)
                .build_combat();
            for _ in 0..3 {
                g.combat_monsters_queue
                    .push(vec![Monster::new(NoopMonster::new(), &mut g.rng)]);
            }
            for _ in 0..8 {
                g.play_card(CardClass::Anger, Some(CreatureRef::monster(0)));
            }
            assert_eq!(g.player.get_status(Status::PenNib), None);
            g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(0)));

            assert_eq!(g.monsters[0].creature.cur_hp, g.monsters[0].creature.max_hp);
            assert_eq!(g.player.get_status(Status::PenNib), Some(1));
            assert_eq!(g.get_relic_value(RelicClass::PenNib), Some(9));
            g.play_card(CardClass::DebugKill, Some(CreatureRef::monster(0)));

            assert_eq!(g.monsters[0].creature.cur_hp, g.monsters[0].creature.max_hp);
            assert_eq!(g.get_relic_value(RelicClass::PenNib), Some(0));
            assert_eq!(g.player.get_status(Status::PenNib), None);
        }
    }

    #[test]
    fn test_tungsten_rod() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::TungstenRod)
            .add_monster(AttackMonster::with_attack_count(4, 2))
            .build_combat();
        let hp = g.player.cur_hp;
        g.play_card(CardClass::Bloodletting, None);
        assert_eq!(g.player.cur_hp, hp - 2);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.cur_hp, hp - 2 - 6);
        g.play_card(CardClass::Defend, None);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.cur_hp, hp - 2 - 6 - 2);

        g.play_card(CardClass::Apparition, None);
        g.play_card(CardClass::Apparition, None);
        g.clear_all_piles();
        g.run_action(GainStatusAction {
            status: Status::PlatedArmor,
            amount: 1,
            target: CreatureRef::player(),
        });
        g.add_card_to_discard_pile(CardClass::BloodForBlood);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.cur_hp, hp - 2 - 6 - 2);
        assert_eq!(g.player.get_status(Status::PlatedArmor), Some(1));
        assert_eq!(g.hand[0].borrow().get_base_cost(), 4);
    }

    #[test]
    fn test_tungsten_rod_buffer() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::TungstenRod)
            .add_player_status(Status::Buffer, 1)
            .add_monster(AttackMonster::new(1))
            .build_combat();
        let hp = g.player.cur_hp;
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.cur_hp, hp);
        assert_eq!(g.player.get_status(Status::Buffer), None);
    }

    #[test]
    fn test_boot() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Boot)
            .add_player_status(Status::Strength, -6)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::Strike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp);
        g.play_card_upgraded(CardClass::Strike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 5);
        g.run_action(GainStatusAction {
            status: Status::Intangible,
            amount: 4,
            target: CreatureRef::monster(0),
        });
        g.play_card_upgraded(CardClass::Strike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 5 - 5);
    }

    #[test]
    fn test_fossilized_helix() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::FossilizedHelix)
            .build_combat();
        assert_eq!(g.player.get_status(Status::Buffer), Some(1));
    }

    #[test]
    fn test_thread_and_needle() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::ThreadAndNeedle)
            .build_combat();
        assert_eq!(g.player.get_status(Status::PlatedArmor), Some(4));
    }

    #[test]
    fn test_chemical_x() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::ChemicalX)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::Whirlwind, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 5 * 5);
        g.play_card(CardClass::Transmutation, None);
        assert_eq!(g.hand.len(), 2);
    }

    #[test]
    fn test_turnip() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Turnip)
            .add_monster(ApplyStatusMonster {
                status: Status::Weak,
                amount: 2,
            })
            .add_monster(ApplyStatusMonster {
                status: Status::Frail,
                amount: 2,
            })
            .build_combat();
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.get_status(Status::Weak), Some(2));
        assert_eq!(g.player.get_status(Status::Frail), None);
        g.player.set_status(Status::Artifact, 2);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.get_status(Status::Weak), Some(1));
        assert_eq!(g.player.get_status(Status::Frail), None);
        assert_eq!(g.player.get_status(Status::Artifact), Some(1));
    }

    #[test]
    fn test_ginger() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Ginger)
            .add_monster(ApplyStatusMonster {
                status: Status::Weak,
                amount: 2,
            })
            .add_monster(ApplyStatusMonster {
                status: Status::Frail,
                amount: 2,
            })
            .build_combat();
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.get_status(Status::Frail), Some(2));
        assert_eq!(g.player.get_status(Status::Weak), None);
        g.player.set_status(Status::Artifact, 2);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.get_status(Status::Frail), Some(1));
        assert_eq!(g.player.get_status(Status::Weak), None);
        assert_eq!(g.player.get_status(Status::Artifact), Some(1));
    }

    #[test]
    fn test_warped_tongs() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::WarpedTongs)
            .build_combat();
        g.add_card_to_draw_pile(CardClass::Strike);

        g.make_move(Move::EndTurn);
        assert_eq!(g.hand[0].borrow().upgrade_count, 1);

        g.make_move(Move::EndTurn);
        assert_eq!(g.hand[0].borrow().upgrade_count, 1);

        g.add_card_to_draw_pile(CardClass::SearingBlow);
        g.make_move(Move::EndTurn);
        assert_eq!(g.get_hand_card(CardClass::Strike).borrow().upgrade_count, 1);
        assert_eq!(
            g.get_hand_card(CardClass::SearingBlow)
                .borrow()
                .upgrade_count,
            1
        );

        g.make_move(Move::EndTurn);
        assert_eq!(g.get_hand_card(CardClass::Strike).borrow().upgrade_count, 1);
        assert_eq!(
            g.get_hand_card(CardClass::SearingBlow)
                .borrow()
                .upgrade_count,
            2
        );

        let mut found_upgraded_strike = false;
        let mut found_upgraded_defend = false;
        for _ in 0..100 {
            g.clear_all_piles();
            g.add_card_to_draw_pile_upgraded(CardClass::WildStrike);
            g.add_card_to_draw_pile(CardClass::Defend);
            g.add_card_to_draw_pile(CardClass::Strike);
            g.make_move(Move::EndTurn);
            assert_eq!(
                g.hand.iter().map(|c| c.borrow().upgrade_count).sum::<i32>(),
                2
            );
            if g.get_hand_card(CardClass::Strike).borrow().upgrade_count == 1 {
                found_upgraded_strike = true;
            }
            if g.get_hand_card(CardClass::Defend).borrow().upgrade_count == 1 {
                found_upgraded_defend = true;
            }
            if found_upgraded_defend && found_upgraded_strike {
                break;
            }
        }
        assert!(found_upgraded_defend && found_upgraded_strike);
    }

    #[test]
    fn test_gambling_chip() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::GamblingChip)
            .add_cards(CardClass::Strike, 10)
            .build_combat();
        assert!(
            g.valid_moves()
                .iter()
                .all(|m| matches!(m, Move::Gamble { .. } | Move::GambleEnd))
        );
    }

    #[test]
    fn test_gambling_chip_warped_tongs() {
        {
            let g = GameBuilder::default()
                .add_relic(RelicClass::GamblingChip)
                .add_relic(RelicClass::WarpedTongs)
                .add_cards(CardClass::Strike, 10)
                .build_combat();
            assert!(g.hand.iter().all(|c| c.borrow().upgrade_count == 0));
        }
        {
            let g = GameBuilder::default()
                .add_relic(RelicClass::WarpedTongs)
                .add_relic(RelicClass::GamblingChip)
                .add_cards(CardClass::Strike, 10)
                .build_combat();
            assert!(g.hand.iter().any(|c| c.borrow().upgrade_count != 0));
        }
    }

    #[test]
    fn test_ceramic_fish() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::CeramicFish)
            .build();
        g.run_action(AddCardToMasterDeckAction(CardClass::Parasite));
        assert_eq!(g.gold, 9);
        g.run_action(AddCardToMasterDeckAction(CardClass::Strike));
        assert_eq!(g.gold, 18);
        g.add_relic(RelicClass::Omamori);
        g.run_action(AddCardToMasterDeckAction(CardClass::Parasite));
        assert_eq!(g.gold, 18);
    }

    #[test]
    fn test_toy_ornithopter() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::ToyOrnithopter)
            .build_combat();
        g.player.cur_hp = 10;
        g.throw_potion(Potion::Ancient, None);
        assert_eq!(g.player.cur_hp, 15);
        g.throw_potion(Potion::Fire, Some(CreatureRef::monster(0)));
        assert_eq!(g.player.cur_hp, 20);
    }

    #[test]
    fn test_necronomicon() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Necronomicon)
            .build_combat();
        let hp = g.monsters[0].creature.cur_hp;
        g.play_card(CardClass::Strike, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6);
        g.play_card(CardClass::Whirlwind, None);
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6 - 5 * 2 * 2);
        assert_eq!(g.energy, 0);

        g.make_move(Move::EndTurn);
        g.energy = 4;
        g.play_card(CardClass::HandOfGreed, Some(CreatureRef::monster(0)));
        assert_eq!(g.monsters[0].creature.cur_hp, hp - 6 - 5 * 2 * 2 - 20 * 2);
        g.play_card(CardClass::HandOfGreed, Some(CreatureRef::monster(0)));
        assert_eq!(
            g.monsters[0].creature.cur_hp,
            hp - 6 - 5 * 2 * 2 - 20 * 2 - 20
        );
        assert_eq!(g.energy, 0);

        g.make_move(Move::EndTurn);
        g.add_card_to_draw_pile(CardClass::HandOfGreed);
        g.play_card_upgraded(CardClass::Havoc, None);
        assert_eq!(g.energy, 3);
        assert_eq!(
            g.monsters[0].creature.cur_hp,
            hp - 6 - 5 * 2 * 2 - 20 * 2 - 20 - 20 * 2
        );
    }

    #[test]
    fn test_necronomicon_equip_unequip() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Necronomicon)
            .build_combat();
        assert_eq!(g.master_deck.len(), 1);
        assert_eq!(g.master_deck[0].borrow().class, CardClass::Necronomicurse);
        g.remove_relic(RelicClass::Necronomicon);
        assert_eq!(g.master_deck.len(), 0);

        g.run_action(AddCardToMasterDeckAction(CardClass::Necronomicurse));
        g.add_relic(RelicClass::Necronomicon);
        assert_eq!(g.master_deck.len(), 2);
        g.remove_relic(RelicClass::Necronomicon);
        assert_eq!(g.master_deck.len(), 1);

        g.master_deck.clear();
        g.add_relic(RelicClass::Omamori);
        g.add_relic(RelicClass::Necronomicon);
        assert_eq!(g.master_deck.len(), 0);
        g.remove_relic(RelicClass::Necronomicon);
        assert_eq!(g.master_deck.len(), 0);
    }

    #[test]
    fn test_enchiridion() {
        let g = GameBuilder::default()
            .add_relic(RelicClass::Enchiridion)
            .add_card(CardClass::Strike)
            .build_combat();
        assert_eq!(g.hand[0].borrow().class.ty(), CardType::Power);
        assert_eq!(g.hand[0].borrow().class.color(), CardColor::Red);
        assert_eq!(g.hand[0].borrow().get_temporary_cost(), Some(0));
    }

    #[test]
    fn test_bloody_idol() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::BloodyIdol)
            .build_combat();
        g.player.cur_hp = 10;
        g.run_action(GainGoldAction(5));
        assert_eq!(g.player.cur_hp, 15);
    }

    #[test]
    fn test_mercury_hourglass() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::MercuryHourglass)
            .add_monster(NoopMonster::new())
            .add_monster(NoopMonster::new())
            .build_combat();
        assert_eq!(
            g.monsters[0].creature.cur_hp,
            g.monsters[0].creature.max_hp - 3
        );
        assert_eq!(
            g.monsters[1].creature.cur_hp,
            g.monsters[1].creature.max_hp - 3
        );
        g.make_move(Move::EndTurn);
        assert_eq!(
            g.monsters[0].creature.cur_hp,
            g.monsters[0].creature.max_hp - 6
        );
        assert_eq!(
            g.monsters[1].creature.cur_hp,
            g.monsters[1].creature.max_hp - 6
        );
    }

    #[test]
    fn test_stone_calendar() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::StoneCalendar)
            .add_monster(NoopMonster::new())
            .add_monster(AttackMonster::new(1))
            .build_combat();
        for _ in 0..6 {
            assert_eq!(g.monsters[0].creature.cur_hp, g.monsters[0].creature.max_hp);
            assert_eq!(g.monsters[1].creature.cur_hp, g.monsters[1].creature.max_hp);
            g.make_move(Move::EndTurn);
        }
        assert_eq!(g.monsters[0].creature.cur_hp, g.monsters[0].creature.max_hp);
        assert_eq!(g.monsters[1].creature.cur_hp, g.monsters[1].creature.max_hp);
        g.monsters[1].creature.cur_hp = 51;
        let hp = g.player.cur_hp;
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.cur_hp, hp);
        assert_eq!(
            g.monsters[0].creature.cur_hp,
            g.monsters[0].creature.max_hp - 52
        );
    }

    #[test]
    fn test_strange_spoon() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::StrangeSpoon)
            .build_combat();
        let mut exhausted = false;
        let mut discarded = false;
        g.play_card(CardClass::FeelNoPain, None);
        for _ in 0..100 {
            g.clear_all_piles();
            g.player.block = 0;
            g.play_card(CardClass::BandageUp, None);
            assert_eq!(g.discard_pile.len() + g.exhaust_pile.len(), 1);
            if !g.discard_pile.is_empty() {
                discarded = true;
                assert_eq!(g.player.block, 0);
            } else {
                exhausted = true;
                assert_ne!(g.player.block, 0);
            }
            if exhausted && discarded {
                break;
            }
        }
        assert!(exhausted && discarded);
    }

    #[test]
    fn test_brimstone() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::Brimstone)
            .build_combat();
        assert_eq!(g.player.get_status(Status::Strength), Some(2));
        assert_eq!(g.monsters[0].creature.get_status(Status::Strength), Some(1));
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.get_status(Status::Strength), Some(4));
        assert_eq!(g.monsters[0].creature.get_status(Status::Strength), Some(2));
    }

    #[test]
    fn test_mummified_hand() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::MummifiedHand)
            .build_combat();
        g.play_card(CardClass::Berserk, None);

        g.add_card_to_hand(CardClass::Strike);
        g.play_card(CardClass::Berserk, None);
        assert_eq!(g.hand[0].borrow().get_temporary_cost(), Some(0));

        for _ in 0..10 {
            g.clear_all_piles();
            g.add_card_to_hand(CardClass::Strike);
            g.hand[0].borrow_mut().set_free_to_play_once();
            g.add_card_to_hand(CardClass::Defend);
            g.play_card(CardClass::Berserk, None);
            assert_eq!(g.hand[1].borrow().get_temporary_cost(), Some(0));

            g.clear_all_piles();
            g.add_card_to_hand(CardClass::Strike);
            g.hand[0].borrow_mut().set_temporary_cost(0);
            g.add_card_to_hand(CardClass::Defend);
            g.play_card(CardClass::Berserk, None);
            assert_eq!(g.hand[1].borrow().get_temporary_cost(), Some(0));

            g.clear_all_piles();
            g.add_card_to_hand(CardClass::Strike);
            g.hand[0].borrow_mut().set_cost(0, None);
            g.add_card_to_hand(CardClass::Defend);
            g.play_card(CardClass::Berserk, None);
            assert_eq!(g.hand[1].borrow().get_temporary_cost(), Some(0));
        }

        let mut discount_0 = false;
        let mut discount_1 = false;
        for _ in 0..100 {
            g.clear_all_piles();
            g.add_card_to_hand(CardClass::Strike);
            g.add_card_to_hand(CardClass::Defend);
            g.play_card(CardClass::Berserk, None);
            assert_ne!(
                g.hand[0].borrow().get_temporary_cost(),
                g.hand[1].borrow().get_temporary_cost()
            );
            if g.hand[0].borrow().get_temporary_cost() == Some(0) {
                discount_0 = true;
            }
            if g.hand[1].borrow().get_temporary_cost() == Some(0) {
                discount_1 = true;
            }
            if discount_0 && discount_1 {
                break;
            }
        }
        assert!(discount_0 && discount_1);
    }

    #[test]
    fn test_centennial_puzzle() {
        {
            let mut g = GameBuilder::default()
                .add_relic(RelicClass::CentennialPuzzle)
                .build_combat();
            g.add_cards_to_draw_pile(CardClass::Strike, 10);
            g.play_card(CardClass::Bloodletting, None);
            assert_eq!(g.hand.len(), 3);
            g.play_card(CardClass::Bloodletting, None);
            assert_eq!(g.hand.len(), 3);
        }
        {
            let mut g = GameBuilder::default()
                .add_cards(CardClass::Strike, 10)
                .add_relic(RelicClass::CentennialPuzzle)
                .add_monster(AttackMonster::new(1))
                .build_combat();
            g.play_card(CardClass::Thunderclap, None);
            g.make_move(Move::EndTurn);
            assert_eq!(g.hand.len(), 8);
            g.make_move(Move::EndTurn);
            assert_eq!(g.hand.len(), 5);
        }
    }

    #[test]
    fn test_self_forming_clay() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::SelfFormingClay)
            .add_monster(AttackMonster::with_attack_count(1, 2))
            .build_combat();
        g.play_card(CardClass::Thunderclap, None);
        assert_eq!(
            g.monsters[0].creature.get_status(Status::NextTurnBlock),
            None
        );
        g.play_card(CardClass::Bloodletting, None);
        assert_eq!(g.player.get_status(Status::NextTurnBlock), Some(3));
        g.make_move(Move::EndTurn);
        g.make_move(Move::EndTurn);
        g.make_move(Move::EndTurn);
        assert_eq!(g.player.block, 6);
    }

    #[test]
    fn test_mark_of_the_bloom() {
        let mut g = GameBuilder::default()
            .add_relic(RelicClass::MarkOfTheBloom)
            .add_relic(RelicClass::LizardTail)
            .add_monster(AttackMonster::new(999))
            .build_combat();
        g.add_potion(Potion::Fairy);
        g.player.cur_hp = 10;
        g.play_card(CardClass::Reaper, None);
        g.play_card(CardClass::BandageUp, None);
        assert_eq!(g.player.cur_hp, 10);
        g.make_move(Move::EndTurn);
        assert_matches!(g.result(), GameStatus::Defeat);
    }

    #[test]
    fn test_lizard_tail() {
        {
            let mut g = GameBuilder::default()
                .add_monster(AttackMonster::new(1000))
                .add_relic(RelicClass::LizardTail)
                .build_combat();
            assert_eq!(g.get_relic_value(RelicClass::LizardTail), Some(1));

            g.make_move(Move::EndTurn);
            assert!(g.player.is_alive());
            assert_eq!(g.player.cur_hp, (g.player.max_hp as f32 * 0.5) as i32);
            assert_eq!(g.get_relic_value(RelicClass::LizardTail), Some(0));
        }
        {
            let mut g = GameBuilder::default()
                .add_monster(AttackMonster::new(1000))
                .add_relic(RelicClass::LizardTail)
                .build_combat();

            g.player.decrease_max_hp(g.player.max_hp - 1);
            g.make_move(Move::EndTurn);
            assert_eq!(g.player.cur_hp, 1);
        }
        {
            let mut g = GameBuilder::default()
                .add_monster(AttackMonster::with_attack_count(1000, 2))
                .add_relic(RelicClass::LizardTail)
                .build_combat();
            g.make_move(Move::EndTurn);
            assert!(!g.player.is_alive());
        }
        {
            // test that fairy is used before lizard tail
            let mut g = GameBuilder::default()
                .add_monster(AttackMonster::new(1000))
                .add_relic(RelicClass::LizardTail)
                .build_combat();
            g.add_potion(Potion::Fairy);
            g.make_move(Move::EndTurn);
            assert_eq!(g.get_relic_value(RelicClass::LizardTail), Some(1));
            assert!(g.potions.iter().all(|p| p.is_none()));
        }
    }

    #[test]
    fn test_champion_belt() {
        {
            let mut g = GameBuilder::default()
                .add_relic(RelicClass::ChampionBelt)
                .build_combat();
            g.play_card(CardClass::Bash, Some(CreatureRef::monster(0)));
            assert_eq!(g.monsters[0].creature.get_status(Status::Weak), Some(1));
            g.play_card(CardClass::Thunderclap, None);
            assert_eq!(g.monsters[0].creature.get_status(Status::Weak), Some(2));
        }
        {
            let mut g = GameBuilder::default()
                .add_relic(RelicClass::ChampionBelt)
                .build_combat();
            g.monsters[0].creature.set_status(Status::Artifact, 1);
            g.play_card(CardClass::Bash, Some(CreatureRef::monster(0)));
            assert_eq!(g.monsters[0].creature.get_status(Status::Weak), None);
            assert_eq!(g.monsters[0].creature.get_status(Status::Artifact), None);
        }
    }
}
