use std::f32::consts::PI;

pub struct Random {
    state: u32,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        let mut rng = Self {
            state: if seed == 0 { 1 } else { seed },
        };
        for _ in 0..16 {
            rng.raw();
        }
        rng
    }

    fn raw(&mut self) -> f32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x as f32 / u32::MAX as f32
    }

    pub fn f32(&mut self, input: usize) -> f32 {
        let std = (2.0 / input as f32).sqrt();

        let u1 = self.raw().max(f32::EPSILON);
        let u2 = self.raw();

        let r = (-2.0 * u1.ln()).sqrt();
        let theta = 2.0 * PI * u2;

        r * theta.cos() * std
    }
}
