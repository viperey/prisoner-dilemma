use uuid::Uuid;

/// How's this strategy going to behave?
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Attitude {
    /// Never defects first. As analyzed & defined by original Thesis' definition.
    Nice,
    /// Non-deterministic cases. Can't tell if it's going to cooperate or defect first.
    Chaos,
    /// Will defect even on the absence of opponent's defection.
    Evil,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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
    OmegaTitForTat,
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
    pub attitude: Attitude,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Prisoner {
    pub id: Uuid,
    pub name: String,
    pub strategy: Strategy,
}
