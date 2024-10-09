use crate::domain::Prisoner;
use crate::strategy::StrategyBuilder;
use uuid::Uuid;

pub struct PrisonerBuilder;
impl PrisonerBuilder {
    pub fn _all() -> Vec<Prisoner> {
        vec![
            PrisonerBuilder::almost_always_cooperate(),
            PrisonerBuilder::almost_always_defect(),
            PrisonerBuilder::almost_always_tit_for_tat(),
            PrisonerBuilder::always_alternate(),
            PrisonerBuilder::always_cooperate(),
            PrisonerBuilder::always_defect(),
            PrisonerBuilder::appease(),
            PrisonerBuilder::copy_average(),
            PrisonerBuilder::equalizer_a(),
            PrisonerBuilder::equalizer_b(),
            PrisonerBuilder::equalizer_c(),
            PrisonerBuilder::equalizer_d(),
            PrisonerBuilder::equalizer_e(),
            PrisonerBuilder::equalizer_f(),
            PrisonerBuilder::extortion_a(),
            PrisonerBuilder::extortion_b(),
            PrisonerBuilder::extortion_c(),
            PrisonerBuilder::extortion_d(),
            PrisonerBuilder::extortion_e(),
            PrisonerBuilder::extortion_f(),
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
            PrisonerBuilder::suspicious_tit_for_tat(),
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

    pub fn almost_always_tit_for_tat() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Tefito".to_string(),
            strategy: StrategyBuilder::almost_always_tit_for_tat(),
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

    pub fn equalizer_a() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Equalizzatore A".to_string(),
            strategy: StrategyBuilder::equalizer_a(),
        }
    }

    pub fn equalizer_b() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Equalizzatore B".to_string(),
            strategy: StrategyBuilder::equalizer_b(),
        }
    }

    pub fn equalizer_c() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Equalizzatore C".to_string(),
            strategy: StrategyBuilder::equalizer_c(),
        }
    }

    pub fn equalizer_d() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Equalizzatore D".to_string(),
            strategy: StrategyBuilder::equalizer_d(),
        }
    }

    pub fn equalizer_e() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Equalizzatore E".to_string(),
            strategy: StrategyBuilder::equalizer_e(),
        }
    }

    pub fn equalizer_f() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Equalizzatore F".to_string(),
            strategy: StrategyBuilder::equalizer_f(),
        }
    }

    pub fn extortion_a() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Extorsione A".to_string(),
            strategy: StrategyBuilder::extortion_a(),
        }
    }

    pub fn extortion_b() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Extorsione B".to_string(),
            strategy: StrategyBuilder::extortion_b(),
        }
    }

    pub fn extortion_c() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Extorsione C".to_string(),
            strategy: StrategyBuilder::extortion_c(),
        }
    }

    pub fn extortion_d() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Extorsione D".to_string(),
            strategy: StrategyBuilder::extortion_d(),
        }
    }

    pub fn extortion_e() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Extorsione E".to_string(),
            strategy: StrategyBuilder::extortion_e(),
        }
    }

    pub fn extortion_f() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Extorsione F".to_string(),
            strategy: StrategyBuilder::extortion_f(),
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

    pub fn suspicious_tit_for_tat() -> Prisoner {
        Prisoner {
            id: Uuid::new_v4(),
            name: "Trueba ".to_string(),
            strategy: StrategyBuilder::suspicious_tit_for_tat(),
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
