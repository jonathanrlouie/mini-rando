use super::randomizer::seed::Seed;

use rand::{Rng, StdRng, SeedableRng};

pub struct GameRng {
    rng: StdRng,
    seed: Seed
}

impl GameRng {
    pub fn new(seed: Seed) -> Self {
        let rng = StdRng::seed_from_u64(seed.get_int_seed_clone());
        GameRng {
            rng,
            seed
        }
    }

    // Don't bother trying to make this pure via move semantics; We get big monad stacks
    // and we have no monad transformers to work with them (plus that's slow)
    pub fn shuffle<T>(&mut self, mut vec: Vec<T>) -> Vec<T> {
        self.rng.shuffle(&mut vec);
        vec
    }
}