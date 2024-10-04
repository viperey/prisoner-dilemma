
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Move {
    Cooperate,
    Defect,
}

#[derive(Debug, Clone)]
pub struct Prisoner {
    pub name: Option<String>,
    pub strategy: StrategyName,
    pub description: String
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum StrategyName {
    Random,
    TitForTat,
    AlwaysCooperate,
    AlwaysDefect,
    Alternate,
    Appease,
    CopyAverage,
    GrimTrigger,
    Pavlovian,
    TitForTwoTats,
    TwoTitsForTat,
    SoftMajo,
    HardMajo,
    PerDDC,
    PerCCB,
    Mistrust,
    HardTitForTat,
    SlowTitForTat,
    Gradual,
    Prober,
    AlmostAlwaysCooperate,
    AlmostAlwaysDefect,
    GenerousTitForTat,
}
