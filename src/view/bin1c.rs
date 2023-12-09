//! Binary scale for [friendly] with 1-char suffixes.
use friendly::scale::{Prefix, PrefixFamily, Scale};

/// A binary scale with 1-char suffixes (K, M, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bin1C {
    pfx: &'static str,
    exp: i32,
}

impl Bin1C {
    const fn new(pfx: &'static str, exp: i32) -> Bin1C {
        Bin1C { pfx, exp }
    }

    pub const UNIT: Bin1C = Bin1C::new("", 0);
    pub const KIBI: Bin1C = Bin1C::new("K", 10);
    pub const MEBI: Bin1C = Bin1C::new("M", 20);
    pub const GIBI: Bin1C = Bin1C::new("G", 30);
    pub const TEBI: Bin1C = Bin1C::new("T", 40);
    pub const PEBI: Bin1C = Bin1C::new("P", 50);
    pub const EXBI: Bin1C = Bin1C::new("E", 60);
    pub const ZEBI: Bin1C = Bin1C::new("Z", 70);
    pub const YOBI: Bin1C = Bin1C::new("Y", 80);

    #[allow(dead_code)]
    pub const AUTO: Scale<Bin1C> = Scale::Auto;

    pub const ALL_PREFIXES: &'static [&'static Bin1C] = &[
        &Bin1C::UNIT,
        &Bin1C::KIBI,
        &Bin1C::MEBI,
        &Bin1C::GIBI,
        &Bin1C::TEBI,
        &Bin1C::PEBI,
        &Bin1C::EXBI,
        &Bin1C::ZEBI,
        &Bin1C::YOBI,
    ];
}

impl Prefix for Bin1C {
    #[inline]
    fn base(&self) -> i32 {
        2
    }

    #[inline]
    fn exponent(&self) -> i32 {
        self.exp
    }

    fn multiplier(&self) -> f64 {
        let mult = 1u128 << self.exp;
        mult as f64
    }

    fn label(&self) -> &'static str {
        self.pfx
    }
}

impl PrefixFamily for Bin1C {
    type Prefix = Bin1C;

    fn unit_prefix() -> Bin1C {
        Bin1C::UNIT
    }

    fn all_prefixes() -> &'static [&'static Bin1C] {
        Bin1C::ALL_PREFIXES
    }
}
