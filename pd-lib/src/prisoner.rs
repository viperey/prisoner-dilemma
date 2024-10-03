use crate::history::History;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyAlmostAlwaysCooperate, StrategyAlmostAlwaysDeflect, StrategyAlternate, StrategyAlwaysCooperate, StrategyAlwaysDefect, StrategyAppease, StrategyCopyAverage, StrategyGenerousTitForTat, StrategyGradual, StrategyGrimTrigger, StrategyHardMajo, StrategyHardTitForTat, StrategyMistrust, StrategyPavlovian, StrategyPerCCD, StrategyPerDDC, StrategyProber, StrategyRandom, StrategySlowTitForTat, StrategySoftMajo, StrategyTitForTat, StrategyTitForTwoTats, StrategyTwoTitsForTat};

pub struct Prisoner {
    pub strategy: Box<dyn Strategy>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Move {
    Cooperate,
    Deflect,
}

impl Prisoner {
    pub fn new(strategy: PrisonerStrategy) -> Prisoner {
        let strategy: Box<dyn Strategy> = match strategy {
            PrisonerStrategy::Random => Box::new(StrategyRandom),
            PrisonerStrategy::TitForTat => Box::new(StrategyTitForTat),
            PrisonerStrategy::AlwaysCooperate => Box::new(StrategyAlwaysCooperate),
            PrisonerStrategy::AlwaysDefect => Box::new(StrategyAlwaysDefect),
            PrisonerStrategy::Alternate => Box::new(StrategyAlternate),
            PrisonerStrategy::Appease => Box::new(StrategyAppease),
            PrisonerStrategy::CopyAverage => Box::new(StrategyCopyAverage),
            PrisonerStrategy::GrimTrigger => Box::new(StrategyGrimTrigger),
            PrisonerStrategy::Pavlovian => Box::new(StrategyPavlovian),
            PrisonerStrategy::TitForTwoTats => Box::new(StrategyTitForTwoTats),
            PrisonerStrategy::TwoTitsForTat => Box::new(StrategyTwoTitsForTat),
            PrisonerStrategy::SoftMajo => Box::new(StrategySoftMajo),
            PrisonerStrategy::HardMajo => Box::new(StrategyHardMajo),
            PrisonerStrategy::PerDDC => Box::new(StrategyPerDDC),
            PrisonerStrategy::PerCCB => Box::new(StrategyPerCCD),
            PrisonerStrategy::Mistrust => Box::new(StrategyMistrust),
            PrisonerStrategy::HardTitForTat => Box::new(StrategyHardTitForTat),
            PrisonerStrategy::SlowTitForTat => Box::new(StrategySlowTitForTat),
            PrisonerStrategy::Gradual => Box::new(StrategyGradual::new()),
            PrisonerStrategy::Prober => Box::new(StrategyProber),
            PrisonerStrategy::AlmostAlwaysCooperate => Box::new(StrategyAlmostAlwaysCooperate),
            PrisonerStrategy::AlmostAlwaysDefect => Box::new(StrategyAlmostAlwaysDeflect),
            PrisonerStrategy::GenerousTitForTat => Box::new(StrategyGenerousTitForTat),
        };
        Prisoner { strategy }
    }

    pub fn decide(&self, history: &History) -> Move {
        self.strategy.decide(history)
    }
}