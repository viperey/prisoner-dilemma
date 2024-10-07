use crate::domain::Prisoner;
use crate::strategy::StrategyBuilder;
use uuid::Uuid;

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
            id: Uuid::new_v4(),
            name: "Chimo ".to_string(),
            strategy: StrategyBuilder::almost_always_cooperate(),
        }
    }

    pub fn almost_always_defect() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Chof ".to_string(),
            strategy: StrategyBuilder::almost_always_defect(),
        }
    }

    pub fn always_alternate() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Altivo ".to_string(),
            strategy: StrategyBuilder::alternate(),
        }
    }

    pub fn always_cooperate() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Coopo ".to_string(),
            strategy: StrategyBuilder::always_cooperate(),
        }
    }

    pub fn always_defect() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Defeo ".to_string(),
            strategy: StrategyBuilder::always_defect(),
        }
    }

    pub fn appease() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Appo ".to_string(), 
            strategy: StrategyBuilder::appease(),
        }
    }

    pub fn copy_average() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Copiota ".to_string(), 
            strategy: StrategyBuilder::copy_average(),
        }
    }

    pub fn generous_tit_for_tat() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Genovevo ".to_string(),
            strategy: StrategyBuilder::generous_tit_for_tat(),
        }
    }

    pub fn gradual() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Topazo ".to_string(), 
            strategy: StrategyBuilder::gradual(),
        }
    }

    pub fn grim_trigger() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Grimaza ".to_string(),
            strategy: StrategyBuilder::grim_trigger(),
        }
    }

    pub fn hard_majo() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Majazo ".to_string(), 
            strategy: StrategyBuilder::hard_majo(),
        }
    }

    pub fn hard_tit_for_tat() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Titan ".to_string(), 
            strategy: StrategyBuilder::hard_tit_for_tat(),
        }
    }

    pub fn mistrust() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Nasty ".to_string(),

            strategy: StrategyBuilder::mistrust(),
        }
    }

    pub fn pavlovian() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Chuchico ".to_string(), 
            strategy: StrategyBuilder::pavlovian(),
        }
    }

    pub fn per_ccd() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Cici ".to_string(),
            strategy: StrategyBuilder::per_ccd(),
        }
    }

    pub fn per_ddc() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Didisi ".to_string(),
            strategy: StrategyBuilder::per_ddc(),
        }
    }

    pub fn prober() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Mentecato ".to_string(), 
            strategy: StrategyBuilder::prober(),
        }
    }

    pub fn random() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Litios ".to_string(),
            strategy: StrategyBuilder::random(),
        }
    }

    pub fn slow_tit_for_tat() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Titin ".to_string(), 
            strategy: StrategyBuilder::slow_tit_for_tat(),
        }
    }

    pub fn soft_majo() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Majin ".to_string(), 
            strategy: StrategyBuilder::soft_majo(),
        }
    }

    pub fn tit_for_tat() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Tito ".to_string(),
            strategy: StrategyBuilder::tit_for_tat(),
        }
    }

    pub fn tit_for_two_tats() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Titatan ".to_string(), 
            strategy: StrategyBuilder::tit_for_two_tats(),
        }
    }

    pub fn two_tits_for_tat() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Tititan ".to_string(),
            strategy: StrategyBuilder::two_tits_for_tat(),
        }
    }

    pub fn from(prisoner: Prisoner) -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: prisoner.name.clone(),
            strategy: prisoner.strategy.clone(),
        }
    }
}
