

pub enum Edge {
    UF,
    UL,
    UB,
    UR,
    DF,
    DL,
    DB,
    DR,
    FR,
    FL,
    BL,
    BR,
}

pub enum Corner {
    UFR,
    UFL,
    UBL,
    UBR,
    DFR,
    DFL,
    DBL,
    DBR,
}

pub enum Center {
    U,  // Up
    D,  // Down
    F,  // Front
    B,  // Back
    L,  // Left
    R,  // Right
}




pub struct Cube {
    pub edges: Vec<Edge>,
    pub corners: Vec<Corner>,
    pub centers: Vec<Center>,
}
