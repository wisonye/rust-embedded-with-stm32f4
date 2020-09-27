#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Hertz(pub u32);

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct KiloHertz(pub u32);

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct MegaHertz(pub u32);

// impl From<MegaHertz> for u32 {
// fn from(value: MegaHertz) -> u32 {
// value.0
// }
// }

impl From<u32> for MegaHertz {
    fn from(value: u32) -> MegaHertz {
        MegaHertz(value / 1_000_000)
    }
}

impl From<u32> for KiloHertz {
    fn from(value: u32) -> KiloHertz {
        KiloHertz(value / 1_000)
    }
}

impl From<u32> for Hertz {
    fn from(value: u32) -> Hertz {
        Hertz(value)
    }
}

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1_000)
    }
}
