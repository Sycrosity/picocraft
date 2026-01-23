pub struct Dimension {
    pub r#type: DimensionType,
}

pub enum DimensionType {
    Overworld,
    Nether,
    TheEnd,
}
