//! Example from: <https://github.com/Absolucy/nanorand-rs/issues/33#issuecomment-1436634311>

pub struct Rand {
    pub seed: u64,
}

const P0: u64 = 0xa076_1d64_78bd_642f;
const P1: u64 = 0xe703_7ed1_a0b4_28db;

impl Rand {
    pub fn rand(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(P0);
        let r = u128::from(self.seed) * u128::from(self.seed ^ P1);
        ((r >> 64) ^ r) as u64
    }

    pub fn range(&mut self, max: u64) -> u64 {
        self.rand() % max
    }
}
