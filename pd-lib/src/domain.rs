use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Move {
    Cooperate,
    Defect,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
pub enum StrategyId {
    Random,
    TitForTat,
    AlwaysCooperate,
    AlwaysDefect,
    Alternate,
    Appease,
    CopyAverage,
    EqualizerA,
    EqualizerB,
    EqualizerC,
    EqualizerD,
    EqualizerE,
    EqualizerF,
    ExtortionA,
    ExtortionB,
    ExtortionC,
    ExtortionD,
    ExtortionE,
    ExtortionF,
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Strategy {
    pub id: StrategyId,
    pub name: String,
    pub description: String,
    pub is_nice: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Prisoner {
    pub id: Uuid,
    pub name: String,
    pub strategy: Strategy,
}
