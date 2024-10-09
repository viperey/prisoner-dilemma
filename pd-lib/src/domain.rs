use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Move {
    Cooperate,
    Defect,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
pub enum StrategyId {
    AlmostAlwaysCooperate,
    AlmostAlwaysDefect,
    AlmostAlwaysTitForTat,
    Alternate,
    AlwaysCooperate,
    AlwaysDefect,
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
    GenerousTitForTat,
    Gradual,
    GrimTrigger,
    HardMajo,
    HardTitForTat,
    Mistrust,
    Pavlovian,
    PerCCD,
    PerDDC,
    Prober,
    Random,
    Reactive,
    SlowTitForTat,
    SoftMajo,
    SuspiciousTitForTat,
    TitForTat,
    TitForTwoTats,
    TwoTitsForTat,
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
