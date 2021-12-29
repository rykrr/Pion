macro_rules! bits {
    ($bit:literal) => (1<<$bit);
    ($bit:literal, $($bits:literal),+) => ((1<<$bit) $(|(1<<$bits))+);
}

pub(crate) use bits;
