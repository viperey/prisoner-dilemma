use crate::domain::{Prisoner, StrategyName};

pub struct PrisonerBuilder;
impl PrisonerBuilder {
    pub fn almost_always_cooperate() -> Prisoner {
        Prisoner {
            name: Some("Chimo".to_string()),
            strategy: StrategyName::AlmostAlwaysCooperate,
            description: "Always cooperate, but make a mistake 10% of the time.".to_string(),
        }
    }

    pub fn almost_always_defect() -> Prisoner {
        Prisoner {
            name: Some("Chof".to_string()),
            strategy: StrategyName::AlmostAlwaysDefect,
            description: "Always defect, but make a mistake 10% of the time.".to_string(),
        }
    }

    pub fn always_alternate() -> Prisoner {
        Prisoner {
            name: Some("Altivo".to_string()),
            strategy: StrategyName::Alternate,
            description: "Alternate between cooperating and defecting.".to_string(),
        }
    }

    pub fn always_cooperate() -> Prisoner {
        Prisoner {
            name: Some("Coopo".to_string()),
            strategy: StrategyName::AlwaysCooperate,
            description: "Always cooperate.".to_string(),
        }
    }

    pub fn always_defect() -> Prisoner {
        Prisoner {
            name: Some("Defeo".to_string()),
            strategy: StrategyName::AlwaysDefect,
            description: "Always defect.".to_string(),
        }
    }

    pub fn appease() -> Prisoner {
        Prisoner {
            name: Some("Appo".to_string()),
            strategy: StrategyName::Appease,
            description: "Start by cooperating, then repeat your previous move if the other player has cooperated or do the opposite if they have defected.".to_string(),
        }
    }

    pub fn copy_average() -> Prisoner {
        Prisoner {
            name: Some("Copiota".to_string()),
            strategy: StrategyName::CopyAverage,
            description: "Choose a random move, but with a probability distribution that matches the other player\'s move distribution. In other words, if the other player has cooperated for 20% of the time, cooperate with a probability of 20%.".to_string(),
        }
    }

    pub fn generous_tit_for_tat() -> Prisoner {
        Prisoner {
            name: Some("Genovevo".to_string()),
            strategy: StrategyName::GenerousTitForTat,
            description: "Start by cooperating, then copy the other player\'s moves.".to_string(),
        }
    }

    pub fn gradual() -> Prisoner {
        Prisoner {
            name: Some("Topazo".to_string()),
            strategy: StrategyName::Gradual,
            description: "Cooperates first move, defects incrementally for each opponent defection, then returns to cooperate after two consecutive cooperations.".to_string(),
        }
    }

    pub fn grim_trigger() -> Prisoner {
        Prisoner {
            name: Some("Grimaza".to_string()),
            strategy: StrategyName::GrimTrigger,
            description: "Cooperate until the other player defects, after that always defect"
                .to_string(),
        }
    }

    pub fn hard_majo() -> Prisoner {
        Prisoner {
            name: Some("Majazo".to_string()),
            strategy: StrategyName::HardMajo,
            description: "Defects initially and defects if the opponent’s defections are greater or equal to cooperations.".to_string(),
        }
    }

    pub fn hard_tit_for_tat() -> Prisoner {
        Prisoner {
            name: Some("Titan".to_string()),
            strategy: StrategyName::HardTitForTat,
            description: "Cooperates first two moves, then defects if the opponent defected in either of the last two moves.".to_string(),
        }
    }

    pub fn mistrust() -> Prisoner {
        Prisoner {
            name: Some("Nasty".to_string()),
            strategy: StrategyName::Mistrust,
            description: "Defects initially, then mimics the opponent's last move.".to_string(),
        }
    }

    pub fn pavlovian() -> Prisoner {
        Prisoner {
            name: Some("Pavlo".to_string()),
            strategy: StrategyName::Pavlovian,
            description: "Pavlov (or Win-stay, Lose-shift). Cooperates if it and its opponent moved alike in previous move and defects if they moved differently.".to_string(),
        }
    }

    pub fn per_ccb() -> Prisoner {
        Prisoner {
            name: Some("Cici".to_string()),
            strategy: StrategyName::PerCCB,
            description: "Plays a periodic sequence of C, C, B.".to_string(),
        }
    }

    pub fn per_ddc() -> Prisoner {
        Prisoner {
            name: Some("Didisi".to_string()),
            strategy: StrategyName::PerDDC,
            description: "Plays a periodic sequence of D, D, C.".to_string(),
        }
    }

    pub fn prober() -> Prisoner {
        Prisoner {
            name: Some("Mentecato".to_string()),
            strategy: StrategyName::Prober,
            description: "Plays D, C, C, then either always defects or plays tit_for_tat based on opponent's second and third responses.".to_string(),
        }
    }

    pub fn random() -> Prisoner {
        Prisoner {
            name: Some("Litios".to_string()),
            strategy: StrategyName::Random,
            description: "Randomly chooses to cooperate or defect.".to_string(),
        }
    }

    pub fn slow_tit_for_tat() -> Prisoner {
        Prisoner {
            name: Some("Titin".to_string()),
            strategy: StrategyName::SlowTitForTat,
            description: "Cooperates first two moves, then defects after two consecutive opponent defections and cooperates after two consecutive cooperations.".to_string(),
        }
    }

    pub fn soft_majo() -> Prisoner {
        Prisoner {
            name: Some("Majin".to_string()),
            strategy: StrategyName::SoftMajo,
            description: "Cooperates as long as the opponent's cooperations are greater than or equal to defections.".to_string(),
        }
    }

    pub fn tit_for_tat() -> Prisoner {
        Prisoner {
            name: Some("Tito".to_string()),
            strategy: StrategyName::TitForTat,
            description: "Start by cooperating, then copy the other player\'s moves.".to_string(),
        }
    }

    pub fn tit_for_two_tats() -> Prisoner {
        Prisoner {
            name: Some("Titatan".to_string()),
            strategy: StrategyName::TitForTwoTats,
            description: "Always cooperate, unless the other player has deflected at least once in the last two moves.".to_string(),
        }
    }

    pub fn two_tits_for_tat() -> Prisoner {
        Prisoner {
            name: Some("Tititan".to_string()),
            strategy: StrategyName::TwoTitsForTat,
            description:
                "Always cooperate, unless the other player has defected the last two times."
                    .to_string(),
        }
    }
}
