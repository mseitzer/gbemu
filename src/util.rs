
macro_rules! get_bit {
    ($v: ident, $bit: expr) => (
        ($v >> $bit) & 0x1
    )
}

macro_rules! set_bit {
    ($v: ident, $bit: expr) => (
        $v | (0x1 << $bit)
    )
}

macro_rules! reset_bit {
    ($v: expr, $bit: expr) => (
        $v & !(0x1 << $bit)
    )
}