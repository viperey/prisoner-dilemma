mod almost_always_cooperate;
mod almost_always_defect;
mod almost_always_tit_for_tat;
mod always_alternate;
mod always_cooperate;
mod always_defect;
mod appease;
mod copy_average;
mod equalizers;
mod extortions;
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
mod suspicious_tit_for_tat;
mod tit_for_tat;
mod tit_for_two_tats;
mod two_tits_for_tat;

use crate::domain::{Move, Prisoner, StrategyId};
use crate::game_result::PartialGameResult;
use crate::strategies::almost_always_cooperate::StrategyAlmostAlwaysCooperate;
use crate::strategies::almost_always_defect::StrategyAlmostAlwaysDefect;
use crate::strategies::almost_always_tit_for_tat::StrategyAlmostAlwaysTitForTat;
use crate::strategies::always_alternate::StrategyAlternate;
use crate::strategies::always_cooperate::StrategyAlwaysCooperate;
use crate::strategies::always_defect::StrategyAlwaysDefect;
use crate::strategies::appease::StrategyAppease;
use crate::strategies::copy_average::StrategyCopyAverage;
use crate::strategies::equalizers::*;
use crate::strategies::extortions::*;
use crate::strategies::generous_tit_for_tat::StrategyGenerousTitForTat;
use crate::strategies::gradual::StrategyGradual;
use crate::strategies::grim_trigger::StrategyGrimTrigger;
use crate::strategies::hard_majo::StrategyHardMajo;
use crate::strategies::hard_tit_for_tat::StrategyHardTitForTat;
use crate::strategies::mistrust::StrategyMistrust;
use crate::strategies::pavlovian::StrategyPavlovian;
use crate::strategies::per_ccd::StrategyPerCCD;
use crate::strategies::per_ddc::StrategyPerDDC;
use crate::strategies::prober::StrategyProber;
use crate::strategies::random::StrategyRandom;
use crate::strategies::slow_tit_for_tat::StrategySlowTitForTat;
use crate::strategies::soft_majo::StrategySoftMajo;
use crate::strategies::suspicious_tit_for_tat::StrategySuspiciousTitForTat;
use crate::strategies::tit_for_tat::StrategyTitForTat;
use crate::strategies::tit_for_two_tats::StrategyTitForTwoTats;
use crate::strategies::two_tits_for_tat::StrategyTwoTitsForTat;
use log::debug;
use std::time::Instant;

pub trait StrategyBehavior {
    fn decide(history: &PartialGameResult) -> Move;
}

pub struct StrategyBehaviorFacade;
impl StrategyBehaviorFacade {
    pub(crate) fn decide(prisoner: &Prisoner, pgr: &PartialGameResult) -> Move {
        let start_time = Instant::now();
        let result = match prisoner.strategy.id {
            StrategyId::AlmostAlwaysCooperate => StrategyAlmostAlwaysCooperate::decide(pgr),
            StrategyId::AlmostAlwaysDefect => StrategyAlmostAlwaysDefect::decide(pgr),
            StrategyId::AlmostAlwaysTitForTat => StrategyAlmostAlwaysTitForTat::decide(pgr),
            StrategyId::Alternate => StrategyAlternate::decide(pgr),
            StrategyId::AlwaysCooperate => StrategyAlwaysCooperate::decide(pgr),
            StrategyId::AlwaysDefect => StrategyAlwaysDefect::decide(pgr),
            StrategyId::Appease => StrategyAppease::decide(pgr),
            StrategyId::CopyAverage => StrategyCopyAverage::decide(pgr),
            StrategyId::EqualizerA => StrategyEqualizerA::decide(pgr),
            StrategyId::EqualizerB => StrategyEqualizerB::decide(pgr),
            StrategyId::EqualizerC => StrategyEqualizerC::decide(pgr),
            StrategyId::EqualizerD => StrategyEqualizerD::decide(pgr),
            StrategyId::EqualizerE => StrategyEqualizerE::decide(pgr),
            StrategyId::EqualizerF => StrategyEqualizerF::decide(pgr),
            StrategyId::ExtortionA => StrategyExtortionA::decide(pgr),
            StrategyId::ExtortionB => StrategyExtortionB::decide(pgr),
            StrategyId::ExtortionC => StrategyExtortionC::decide(pgr),
            StrategyId::ExtortionD => StrategyExtortionD::decide(pgr),
            StrategyId::ExtortionE => StrategyExtortionE::decide(pgr),
            StrategyId::ExtortionF => StrategyExtortionF::decide(pgr),
            StrategyId::GenerousTitForTat => StrategyGenerousTitForTat::decide(pgr),
            StrategyId::Gradual => StrategyGradual::decide(pgr),
            StrategyId::GrimTrigger => StrategyGrimTrigger::decide(pgr),
            StrategyId::HardMajo => StrategyHardMajo::decide(pgr),
            StrategyId::HardTitForTat => StrategyHardTitForTat::decide(pgr),
            StrategyId::Mistrust => StrategyMistrust::decide(pgr),
            StrategyId::Pavlovian => StrategyPavlovian::decide(pgr),
            StrategyId::PerCCD => StrategyPerCCD::decide(pgr),
            StrategyId::PerDDC => StrategyPerDDC::decide(pgr),
            StrategyId::Prober => StrategyProber::decide(pgr),
            StrategyId::Random => StrategyRandom::decide(pgr),
            StrategyId::SlowTitForTat => StrategySlowTitForTat::decide(pgr),
            StrategyId::SoftMajo => StrategySoftMajo::decide(pgr),
            StrategyId::SuspiciousTitForTat => StrategySuspiciousTitForTat::decide(pgr),
            StrategyId::TitForTat => StrategyTitForTat::decide(pgr),
            StrategyId::TitForTwoTats => StrategyTitForTwoTats::decide(pgr),
            StrategyId::TwoTitsForTat => StrategyTwoTitsForTat::decide(pgr),
        };
        let end_time = Instant::now();
        debug!(
            "Strategy {:#?} took {:.3} seconds to decide",
            prisoner.strategy,
            end_time.duration_since(start_time).as_secs_f64()
        );
        result
    }
}

pub mod utils {
    use rand::Rng;

    use crate::domain::Move;

    pub fn decide(probability: f64) -> Move {
        let mut rng = rand::thread_rng();
        let is_cooperate: bool = rng.gen_bool(probability);
        match is_cooperate {
            true => Move::Cooperate,
            false => Move::Defect,
        }
    }
}
