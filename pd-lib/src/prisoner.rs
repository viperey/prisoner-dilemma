use crate::strategy::StrategyName;
use rand::distributions::Alphanumeric;
use rand::Rng;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Prisoner {
    pub name: String,
    pub surname: Option<String>,
    pub strategy: StrategyName,
    pub description: String,
}

pub struct PrisonerBuilder;
impl PrisonerBuilder {
    pub fn _all() -> Vec<Prisoner> {
        vec![
            PrisonerBuilder::almost_always_cooperate(),
            PrisonerBuilder::almost_always_defect(),
            PrisonerBuilder::always_alternate(),
            PrisonerBuilder::always_cooperate(),
            PrisonerBuilder::always_defect(),
            PrisonerBuilder::appease(),
            PrisonerBuilder::copy_average(),
            PrisonerBuilder::generous_tit_for_tat(),
            PrisonerBuilder::gradual(),
            PrisonerBuilder::grim_trigger(),
            PrisonerBuilder::hard_majo(),
            PrisonerBuilder::hard_tit_for_tat(),
            PrisonerBuilder::mistrust(),
            PrisonerBuilder::pavlovian(),
            PrisonerBuilder::per_ccd(),
            PrisonerBuilder::per_ddc(),
            PrisonerBuilder::prober(),
            PrisonerBuilder::random(),
            PrisonerBuilder::slow_tit_for_tat(),
            PrisonerBuilder::soft_majo(),
            PrisonerBuilder::tit_for_two_tats(),
            PrisonerBuilder::tit_for_tat(),
            PrisonerBuilder::two_tits_for_tat(),
        ]
    }

    pub fn almost_always_cooperate() -> Prisoner {
        Prisoner {
            name: "Chimo ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::AlmostAlwaysCooperate,
            description: "Always cooperate, but make a mistake 10% of the time.".to_string(),
        }
    }

    pub fn almost_always_defect() -> Prisoner {
        Prisoner {
            name: "Chof ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::AlmostAlwaysDefect,
            description: "Always defect, but make a mistake 10% of the time.".to_string(),
        }
    }

    pub fn always_alternate() -> Prisoner {
        Prisoner {
            name: "Altivo ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::Alternate,
            description: "Alternate between cooperating and defecting.".to_string(),
        }
    }

    pub fn always_cooperate() -> Prisoner {
        Prisoner {
            name: "Coopo ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::AlwaysCooperate,
            description: "Always cooperate.".to_string(),
        }
    }

    pub fn always_defect() -> Prisoner {
        Prisoner {
            name: "Defeo ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::AlwaysDefect,
            description: "Always defect.".to_string(),
        }
    }

    pub fn appease() -> Prisoner {
        Prisoner {
            name: "Appo ".to_string(), surname: Some(Self::get_surname()),
            strategy: StrategyName::Appease,
            description: "Start by cooperating, then repeat your previous move if the other player has cooperated or do the opposite if they have defected.".to_string(),
        }
    }

    pub fn copy_average() -> Prisoner {
        Prisoner {
            name: "Copiota ".to_string(), surname: Some(Self::get_surname()),
            strategy: StrategyName::CopyAverage,
            description: "Choose a random move, but with a probability distribution that matches the other player\'s move distribution. In other words, if the other player has cooperated for 20% of the time, cooperate with a probability of 20%.".to_string(),
        }
    }

    pub fn generous_tit_for_tat() -> Prisoner {
        Prisoner {
            name: "Genovevo ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::GenerousTitForTat,
            description: "Start by cooperating, then copy the other player\'s moves.".to_string(),
        }
    }

    pub fn gradual() -> Prisoner {
        Prisoner {
            name: "Topazo ".to_string(), surname: Some(Self::get_surname()),
            strategy: StrategyName::Gradual,
            description: "Cooperates first move, defects incrementally for each opponent defection, then returns to cooperate after two consecutive cooperations.".to_string(),
        }
    }

    pub fn grim_trigger() -> Prisoner {
        Prisoner {
            name: "Grimaza ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::GrimTrigger,
            description: "Cooperate until the other player defects, after that always defect"
                .to_string(),
        }
    }

    pub fn hard_majo() -> Prisoner {
        Prisoner {
            name: "Majazo ".to_string(), surname: Some(Self::get_surname()),
            strategy: StrategyName::HardMajo,
            description: "Defects initially and defects if the opponent’s defections are greater or equal to cooperations.".to_string(),
        }
    }

    pub fn hard_tit_for_tat() -> Prisoner {
        Prisoner {
            name: "Titan ".to_string(), surname: Some(Self::get_surname()),
            strategy: StrategyName::HardTitForTat,
            description: "Cooperates first two moves, then defects if the opponent defected in either of the last two moves.".to_string(),
        }
    }

    pub fn mistrust() -> Prisoner {
        Prisoner {
            name: "Nasty ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::Mistrust,
            description: "Defects initially, then mimics the opponent's last move.".to_string(),
        }
    }

    pub fn pavlovian() -> Prisoner {
        Prisoner {
            name: "Chuchico ".to_string(), surname: Some(Self::get_surname()),
            strategy: StrategyName::Pavlovian,
            description: "Pavlov (or Win-stay, Lose-shift). Cooperates if it and its opponent moved alike in previous move and defects if they moved differently.".to_string(),
        }
    }

    pub fn per_ccd() -> Prisoner {
        Prisoner {
            name: "Cici ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::PerCCD,
            description: "Plays a periodic sequence of C, C, D.".to_string(),
        }
    }

    pub fn per_ddc() -> Prisoner {
        Prisoner {
            name: "Didisi ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::PerDDC,
            description: "Plays a periodic sequence of D, D, C.".to_string(),
        }
    }

    pub fn prober() -> Prisoner {
        Prisoner {
            name: "Mentecato ".to_string(), surname: Some(Self::get_surname()),
            strategy: StrategyName::Prober,
            description: "Plays D, C, C, then either always defects or plays tit_for_tat based on opponent's second and third responses.".to_string(),
        }
    }

    pub fn random() -> Prisoner {
        Prisoner {
            name: "Litios ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::Random,
            description: "Randomly chooses to cooperate or defect.".to_string(),
        }
    }

    pub fn slow_tit_for_tat() -> Prisoner {
        Prisoner {
            name: "Titin ".to_string(), surname: Some(Self::get_surname()),
            strategy: StrategyName::SlowTitForTat,
            description: "Cooperates first two moves, then defects after two consecutive opponent defections and cooperates after two consecutive cooperations.".to_string(),
        }
    }

    pub fn soft_majo() -> Prisoner {
        Prisoner {
            name: "Majin ".to_string(), surname: Some(Self::get_surname()),
            strategy: StrategyName::SoftMajo,
            description: "Cooperates as long as the opponent's cooperations are greater than or equal to defections.".to_string(),
        }
    }

    pub fn tit_for_tat() -> Prisoner {
        Prisoner {
            name: "Tito ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::TitForTat,
            description: "Start by cooperating, then copy the other player\'s moves.".to_string(),
        }
    }

    pub fn tit_for_two_tats() -> Prisoner {
        Prisoner {
            name: "Titatan ".to_string(), surname: Some(Self::get_surname()),
            strategy: StrategyName::TitForTwoTats,
            description: "Always cooperate, unless the other player has deflected at least once in the last two moves.".to_string(),
        }
    }

    pub fn two_tits_for_tat() -> Prisoner {
        Prisoner {
            name: "Tititan ".to_string(),
            surname: Some(Self::get_surname()),
            strategy: StrategyName::TwoTitsForTat,
            description:
                "Always cooperate, unless the other player has defected the last two times."
                    .to_string(),
        }
    }

    pub fn from(prisoner: Prisoner) -> Prisoner {
        Prisoner {
            name: prisoner.name.clone(),
            surname: Some(Self::get_surname()),
            strategy: prisoner.strategy.clone(),
            description: prisoner.description.clone(),
        }
    }

    fn get_surname() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }
}
