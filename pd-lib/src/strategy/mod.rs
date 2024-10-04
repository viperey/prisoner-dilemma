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

use crate::domain::{Move, Prisoner, StrategyName};
use crate::game_result::PartialGameResult;
use crate::strategy::almost_always_cooperate::StrategyAlmostAlwaysCooperate;
use crate::strategy::almost_always_defect::StrategyAlmostAlwaysDefect;
use crate::strategy::generous_tit_for_tat::StrategyGenerousTitForTat;
use crate::strategy::random::StrategyRandom;
use crate::strategy::tit_for_tat::StrategyTitForTat;
use crate::strategy::always_alternate::StrategyAlternate;
use crate::strategy::always_cooperate::StrategyAlwaysCooperate;
use crate::strategy::always_defect::StrategyAlwaysDefect;
use crate::strategy::appease::StrategyAppease;
use crate::strategy::copy_average::StrategyCopyAverage;
use crate::strategy::gradual::StrategyGradual;
use crate::strategy::grim_trigger::StrategyGrimTrigger;
use crate::strategy::hard_majo::StrategyHardMajo;
use crate::strategy::hard_tit_for_tat::StrategyHardTitForTat;
use crate::strategy::mistrust::StrategyMistrust;
use crate::strategy::pavlovian::StrategyPavlovian;
use crate::strategy::per_ccd::StrategyPerCCD;
use crate::strategy::per_ddc::StrategyPerDDC;
use crate::strategy::prober::StrategyProber;
use crate::strategy::slow_tit_for_tat::StrategySlowTitForTat;
use crate::strategy::soft_majo::StrategySoftMajo;
use crate::strategy::tit_for_two_tats::StrategyTitForTwoTats;
use crate::strategy::two_tits_for_tat::StrategyTwoTitsForTat;

pub trait StrategyTrait {
    fn decide(history: &PartialGameResult) -> Move;
}

pub struct StrategyFacade;
impl StrategyFacade {
    pub(crate) fn decide(prisoner_v2: &Prisoner, history: &PartialGameResult) -> Move {
        match prisoner_v2.strategy {
            StrategyName::AlmostAlwaysCooperate => StrategyAlmostAlwaysCooperate::decide(history),
            StrategyName::AlmostAlwaysDefect => StrategyAlmostAlwaysDefect::decide(history),
            StrategyName::Alternate => StrategyAlternate::decide(history),
            StrategyName::AlwaysCooperate => StrategyAlwaysCooperate::decide(history),
            StrategyName::AlwaysDefect => StrategyAlwaysDefect::decide(history),
            StrategyName::Appease => StrategyAppease::decide(history),
            StrategyName::CopyAverage => StrategyCopyAverage::decide(history),
            StrategyName::GenerousTitForTat => StrategyGenerousTitForTat::decide(history),
            StrategyName::Gradual => StrategyGradual::decide(history),
            StrategyName::GrimTrigger => StrategyGrimTrigger::decide(history),
            StrategyName::HardMajo => StrategyHardMajo::decide(history),
            StrategyName::HardTitForTat => StrategyHardTitForTat::decide(history),
            StrategyName::Mistrust => StrategyMistrust::decide(history),
            StrategyName::Pavlovian => StrategyPavlovian::decide(history),
            StrategyName::PerCCB => StrategyPerCCD::decide(history),
            StrategyName::PerDDC => StrategyPerDDC::decide(history),
            StrategyName::Prober => StrategyProber::decide(history),
            StrategyName::Random => StrategyRandom::decide(history),
            StrategyName::SlowTitForTat => StrategySlowTitForTat::decide(history),
            StrategyName::SoftMajo => StrategySoftMajo::decide(history),
            StrategyName::TitForTat => StrategyTitForTat::decide(history),
            StrategyName::TitForTwoTats => StrategyTitForTwoTats::decide(history),
            StrategyName::TwoTitsForTat => StrategyTwoTitsForTat::decide(history),
        }
    }
}
