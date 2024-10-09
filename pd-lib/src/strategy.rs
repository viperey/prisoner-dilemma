use crate::domain::{Strategy, StrategyId};

pub struct StrategyBuilder;

impl StrategyBuilder {
    pub fn almost_always_cooperate() -> Strategy {
        Strategy {
            id: StrategyId::AlmostAlwaysCooperate,
            name: "Almost Always Cooperate".to_string(),
            description: "Always cooperate, but make a mistake 10% of the time.".to_string(),
            is_nice: true,
        }
    }

    pub fn almost_always_defect() -> Strategy {
        Strategy {
            id: StrategyId::AlmostAlwaysDefect,
            name: "Almost Always Defect".to_string(),
            description: "Always defect, but make a mistake 10% of the time.".to_string(),
            is_nice: false,
        }
    }

    pub fn alternate() -> Strategy {
        Strategy {
            id: StrategyId::Alternate,
            name: "Alternate".to_string(),
            description: "Alternate between cooperating and deflecting.".to_string(),
            is_nice: true,
        }
    }

    pub fn always_cooperate() -> Strategy {
        Strategy {
            id: StrategyId::AlwaysCooperate,
            name: "Always Cooperate".to_string(),
            description: "Always cooperate.".to_string(),
            is_nice: true,
        }
    }

    pub fn always_defect() -> Strategy {
        Strategy {
            id: StrategyId::AlwaysDefect,
            name: "Always Defect".to_string(),
            description: "Always defect.".to_string(),
            is_nice: false,
        }
    }

    pub fn appease() -> Strategy {
        Strategy {
            id: StrategyId::Appease,
            name: "Appease".to_string(),
            description: "Start by cooperating, then repeat your previous move if the other player has cooperated or do the opposite if they have defected.".to_string(),
            is_nice: true,
        }
    }

    pub fn copy_average() -> Strategy {
        Strategy {
            id: StrategyId::CopyAverage,
            name: "Copy Average".to_string(),
            description: "Choose a random move, but with a probability distribution that matches the other player's move distribution. In other words, if the other player has cooperated for 20% of the time, cooperate with a probability of 20%.".to_string(),
            is_nice: true,
        }
    }

    pub fn equalizer_a() -> Strategy {
        Strategy {
            id: StrategyId::EqualizerA,
            name: "Equalizer A".to_string(),
            description: "Start cooperating, then play with probabilities cc = 3/4 cd = 1/4 dc = 1/2 dd = 1/4 given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn equalizer_b() -> Strategy {
        Strategy {
            id: StrategyId::EqualizerB,
            name: "Equalizer B".to_string(),
            description: "Start cooperating, then play with probabilities cc = 9/10 cd = 7/10 dc = 1/5 dd = 1/10 given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn equalizer_c() -> Strategy {
        Strategy {
            id: StrategyId::EqualizerC,
            name: "Equalizer C".to_string(),
            description: "Start cooperating, then play with probabilities cc = 9/10 cd = 1/2 dc = 1/2 dd = 3/10 given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn equalizer_d() -> Strategy {
        Strategy {
            id: StrategyId::EqualizerD,
            name: "Equalizer D".to_string(),
            description: "Start cooperating, then play with probabilities cc 27/35 cd = 17/35 dc = 1/5 dd = 2/35 given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn equalizer_e() -> Strategy {
        Strategy {
            id: StrategyId::EqualizerE,
            name: "Equalizer E".to_string(),
            description: "Start cooperating, then play with probabilities cc = 2/3 cd = 0 dc = 2/3 dd = 1/ given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn equalizer_f() -> Strategy {
        Strategy {
            id: StrategyId::EqualizerF,
            name: "Equalizer F".to_string(),
            description: "Start cooperating, then play with probabilities cc = 1 cd = 13/15 dc = 1/5 dd = 2/5 given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn extortion_a() -> Strategy {
        Strategy {
            id: StrategyId::ExtortionA,
            name: "Extortion A".to_string(),
            description: "Start by cooperating, then play with probabilities cc = 8/9 cd = 2/9 dc = 11/18 dd = 0 given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn extortion_b() -> Strategy {
        Strategy {
            id: StrategyId::ExtortionB,
            name: "Extortion B".to_string(),
            description: "Start by cooperating, then play with probabilities cc = 0.8 cd = 0.1 dc = 0.6 dd = 0 given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn extortion_c() -> Strategy {
        Strategy {
            id: StrategyId::ExtortionC,
            name: "Extortion C".to_string(),
            description: "Start by cooperating, then play with probabilities cc = 11/12 cd = 5/24 dc = 2/3 dd = 1/24 given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn extortion_d() -> Strategy {
        Strategy {
            id: StrategyId::ExtortionD,
            name: "Extortion D".to_string(),
            description: "Start by cooperating, then play with probabilities cc = 5/6 cd = 0.2 dc = 0.5 dd = 0 given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn extortion_e() -> Strategy {
        Strategy {
            id: StrategyId::ExtortionE,
            name: "Extortion E".to_string(),
            description: "Start by cooperating, then play with probabilities cc = 17/20 cd = 3/40 dc = 0.7 dd = 0 given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn extortion_f() -> Strategy {
        Strategy {
            id: StrategyId::ExtortionF,
            name: "Extortion F".to_string(),
            description: "Start by cooperating, then play with probabilities cc = 11/15 cd = 2/15 dc = 7/15 dd = 0 given a past movement".to_string(),
            is_nice: true,
        }
    }

    pub fn generous_tit_for_tat() -> Strategy {
        Strategy {
            id: StrategyId::GenerousTitForTat,
            name: "Generous Tit For Tat".to_string(),
            description: "Start by cooperating, then copy the other player's moves.".to_string(),
            is_nice: true,
        }
    }

    pub fn gradual() -> Strategy {
        Strategy {
            id: StrategyId::Gradual,
            name: "Gradual".to_string(),
            description: "Cooperates first move, defects incrementally for each opponent defection, then returns to cooperate after two consecutive cooperations.".to_string(),
            is_nice: true,
        }
    }

    pub fn grim_trigger() -> Strategy {
        Strategy {
            id: StrategyId::GrimTrigger,
            name: "Grim Trigger".to_string(),
            description: "Cooperate until the other player defects, after that always defect"
                .to_string(),
            is_nice: true,
        }
    }

    pub fn hard_majo() -> Strategy {
        Strategy {
            id: StrategyId::HardMajo,
            name: "Hard Majo".to_string(),
            description: "Defects initially and defects if the opponent’s defections are greater or equal to cooperations.".to_string(),
            is_nice: false,
        }
    }

    pub fn hard_tit_for_tat() -> Strategy {
        Strategy {
            id: StrategyId::HardTitForTat,
            name: "Hard Tit For Tat".to_string(),
            description: "Cooperates first two moves, then defects if the opponent defected in either of the last two moves.".to_string(),
            is_nice: true,
        }
    }

    pub fn mistrust() -> Strategy {
        Strategy {
            id: StrategyId::Mistrust,
            name: "Mistrust".to_string(),
            description: "Defects initially, then mimics the opponent's last move.".to_string(),
            is_nice: false,
        }
    }

    pub fn pavlovian() -> Strategy {
        Strategy {
            id: StrategyId::Pavlovian,
            name: "Pavlovian".to_string(),
            description: "Pavlov (or Win-stay, Lose-shift). Cooperates if it and its opponent moved alike in previous move and defects if they moved differently.".to_string(),
            is_nice: true,
        }
    }

    pub fn per_ccd() -> Strategy {
        Strategy {
            id: StrategyId::PerCCD,
            name: "Per CCB".to_string(),
            description: "Plays a periodic sequence of C, C, B.".to_string(),
            is_nice: true,
        }
    }

    pub fn per_ddc() -> Strategy {
        Strategy {
            id: StrategyId::PerDDC,
            name: "Per DDC".to_string(),
            description: "Plays a periodic sequence of D, D, C.".to_string(),
            is_nice: true,
        }
    }

    pub fn prober() -> Strategy {
        Strategy {
            id: StrategyId::Prober,
            name: "Prober".to_string(),
            description: "Plays D, C, C, then either always defects or plays tit_for_tat based on opponent's second and third responses.".to_string(),            is_nice: true,
        }
    }

    pub fn random() -> Strategy {
        Strategy {
            id: StrategyId::Random,
            name: "Random".to_string(),
            description: "Randomly chooses to cooperate or deflect.".to_string(),
            is_nice: true,
        }
    }

    pub fn slow_tit_for_tat() -> Strategy {
        Strategy {
            id: StrategyId::SlowTitForTat,
            name: "Slow Tit For Tat".to_string(),
            description: "Cooperates first two moves, then defects after two consecutive opponent defections and cooperates after two consecutive cooperations.".to_string(),
            is_nice: true,
        }
    }

    pub fn soft_majo() -> Strategy {
        Strategy {
            id: StrategyId::SoftMajo,
            name: "Soft Majo".to_string(),
            description: "Cooperates as long as the opponent's cooperations are greater than or equal to defections.".to_string(),
            is_nice: true,
        }
    }

    pub fn suspicious_tit_for_tat() -> Strategy {
        Strategy {
            id: StrategyId::SuspiciousTitForTat,
            name: "Trueba".to_string(),
            description:
                "Defects on the first round and imitates its opponent's previous move thereafter."
                    .to_string(),
            is_nice: false,
        }
    }

    pub fn tit_for_tat() -> Strategy {
        Strategy {
            id: StrategyId::TitForTat,
            name: "Tit For Tat".to_string(),
            description: "Start by cooperating, then copy the other player's moves.".to_string(),
            is_nice: true,
        }
    }

    pub fn tit_for_two_tats() -> Strategy {
        Strategy {
            id: StrategyId::TitForTwoTats,
            name: "Tit For Two Tats".to_string(),
            description: "Always cooperate, unless the other player has deflected at least once in the last two moves.".to_string(),
            is_nice: true,
        }
    }

    pub fn two_tits_for_tat() -> Strategy {
        Strategy {
            id: StrategyId::TwoTitsForTat,
            name: "Two Tits For Tat".to_string(),
            description:
                "Always cooperate, unless the other player has defected the last two times."
                    .to_string(),
            is_nice: true,
        }
    }
}
