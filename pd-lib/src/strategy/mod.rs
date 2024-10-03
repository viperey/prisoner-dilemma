mod tit_for_tat;
mod random;
mod always_cooperate;
mod always_defect;
mod always_alternate;
mod appease;
mod two_tits_for_tat;
mod tit_for_two_tats;
mod pavlovian;
mod grim_trigger;
mod copy_average;
mod soft_majo;
mod hard_majo;
mod per_ddc;
mod per_ccb;
mod mistrust;
mod hard_tit_for_tat;
mod slow_tit_for_tat;
mod gradual;
mod prober;
mod almost_always_cooperate;
mod almost_always_deflect;
mod generous_tit_for_tat;

use crate::prisoner::Move;
use crate::history::History;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum PrisonerStrategy {
    Random,
    TitForTat,
    AlwaysCooperate,
    AlwaysDefect,
    Alternate,
    Appease,
    CopyAverage,
    GrimTrigger,
    Pavlovian,
    TitForTwoTats,
    TwoTitsForTat,
    SoftMajo,
    HardMajo,
    PerDDC,
    PerCCB,
    Mistrust,
    HardTitForTat,
    SlowTitForTat,
    Gradual,
    Prober,
    AlmostAlwaysCooperate,
    AlmostAlwaysDefect,
    GenerousTitForTat,
}


pub trait Strategy {
    fn decide(&self, history: &History) -> Move;
    fn name(&self) -> PrisonerStrategy;
    fn description(&self) -> String;
    fn nicesness_score(&self) -> f64;
}

pub struct StrategyTitForTat;
pub struct StrategyRandom;
pub struct StrategyAlwaysCooperate;
pub struct StrategyAlwaysDefect;
pub struct StrategyAlternate;
pub struct StrategyAppease;
pub struct StrategyCopyAverage;
pub struct StrategyGrimTrigger;
pub struct StrategyPavlovian;
pub struct StrategyTitForTwoTats;
pub struct StrategyTwoTitsForTat;
pub struct StrategySoftMajo;
pub struct StrategyHardMajo;
pub struct StrategyPerDDC;
pub struct StrategyPerCCD;
pub struct StrategyMistrust;
pub struct StrategyHardTitForTat;
pub struct StrategySlowTitForTat;
pub struct StrategyProber;
pub struct StrategyGradual {
    defect_counter: usize,
}
pub struct StrategyAlmostAlwaysCooperate;
pub struct StrategyAlmostAlwaysDeflect;
pub struct StrategyGenerousTitForTat;

