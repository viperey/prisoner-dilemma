mod almost_always_cooperate;
mod almost_always_defect;
mod always_alternate;
mod always_cooperate;
mod always_defect;
mod appease;
mod copy_average;
mod generous_tit_for_tat;
mod gradual;
mod grim_trigger;
mod hard_majo;
mod hard_tit_for_tat;
mod mistrust;
mod pavlovian;
mod per_ccd;
mod per_ddc;
mod prober;
mod random;
mod slow_tit_for_tat;
mod soft_majo;
mod tit_for_tat;
mod tit_for_two_tats;
mod two_tits_for_tat;

use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::prisoner::Prisoner;
use crate::strategy::almost_always_cooperate::StrategyAlmostAlwaysCooperate;
use crate::strategy::almost_always_defect::StrategyAlmostAlwaysDefect;
use crate::strategy::always_alternate::StrategyAlternate;
use crate::strategy::always_cooperate::StrategyAlwaysCooperate;
use crate::strategy::always_defect::StrategyAlwaysDefect;
use crate::strategy::appease::StrategyAppease;
use crate::strategy::copy_average::StrategyCopyAverage;
use crate::strategy::generous_tit_for_tat::StrategyGenerousTitForTat;
use crate::strategy::gradual::StrategyGradual;
use crate::strategy::grim_trigger::StrategyGrimTrigger;
use crate::strategy::hard_majo::StrategyHardMajo;
use crate::strategy::hard_tit_for_tat::StrategyHardTitForTat;
use crate::strategy::mistrust::StrategyMistrust;
use crate::strategy::pavlovian::StrategyPavlovian;
use crate::strategy::per_ccd::StrategyPerCCD;
use crate::strategy::per_ddc::StrategyPerDDC;
use crate::strategy::prober::StrategyProber;
use crate::strategy::random::StrategyRandom;
use crate::strategy::slow_tit_for_tat::StrategySlowTitForTat;
use crate::strategy::soft_majo::StrategySoftMajo;
use crate::strategy::tit_for_tat::StrategyTitForTat;
use crate::strategy::tit_for_two_tats::StrategyTitForTwoTats;
use crate::strategy::two_tits_for_tat::StrategyTwoTitsForTat;
use log::debug;
use std::time::Instant;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum StrategyName {
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
    PerCCD,
    Mistrust,
    HardTitForTat,
    SlowTitForTat,
    Gradual,
    Prober,
    AlmostAlwaysCooperate,
    AlmostAlwaysDefect,
    GenerousTitForTat,
}

pub trait StrategyTrait {
    fn decide(history: &PartialGameResult) -> Move;
}

pub struct StrategyFacade;
impl StrategyFacade {
    pub(crate) fn decide(prisoner_v2: &Prisoner, pgr: &PartialGameResult) -> Move {
        let start_time = Instant::now();
        let result = match prisoner_v2.strategy {
            StrategyName::AlmostAlwaysCooperate => StrategyAlmostAlwaysCooperate::decide(pgr),
            StrategyName::AlmostAlwaysDefect => StrategyAlmostAlwaysDefect::decide(pgr),
            StrategyName::Alternate => StrategyAlternate::decide(pgr),
            StrategyName::AlwaysCooperate => StrategyAlwaysCooperate::decide(pgr),
            StrategyName::AlwaysDefect => StrategyAlwaysDefect::decide(pgr),
            StrategyName::Appease => StrategyAppease::decide(pgr),
            StrategyName::CopyAverage => StrategyCopyAverage::decide(pgr),
            StrategyName::GenerousTitForTat => StrategyGenerousTitForTat::decide(pgr),
            StrategyName::Gradual => StrategyGradual::decide(pgr),
            StrategyName::GrimTrigger => StrategyGrimTrigger::decide(pgr),
            StrategyName::HardMajo => StrategyHardMajo::decide(pgr),
            StrategyName::HardTitForTat => StrategyHardTitForTat::decide(pgr),
            StrategyName::Mistrust => StrategyMistrust::decide(pgr),
            StrategyName::Pavlovian => StrategyPavlovian::decide(pgr),
            StrategyName::PerCCD => StrategyPerCCD::decide(pgr),
            StrategyName::PerDDC => StrategyPerDDC::decide(pgr),
            StrategyName::Prober => StrategyProber::decide(pgr),
            StrategyName::Random => StrategyRandom::decide(pgr),
            StrategyName::SlowTitForTat => StrategySlowTitForTat::decide(pgr),
            StrategyName::SoftMajo => StrategySoftMajo::decide(pgr),
            StrategyName::TitForTat => StrategyTitForTat::decide(pgr),
            StrategyName::TitForTwoTats => StrategyTitForTwoTats::decide(pgr),
            StrategyName::TwoTitsForTat => StrategyTwoTitsForTat::decide(pgr),
        };
        let end_time = Instant::now();
        debug!(
            "Strategy {:#?} took {:.3} seconds to decide",
            prisoner_v2.strategy, end_time.duration_since(start_time).as_secs_f64()
        );
        result
    }
}
