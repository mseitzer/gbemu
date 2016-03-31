
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum Reg8 {
    A = 0,
    F = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    H = 6,
    L = 7
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum Reg16 {
    AF = 1,
    BC = 3,
    DE = 5,
    HL = 7,
    SP
}