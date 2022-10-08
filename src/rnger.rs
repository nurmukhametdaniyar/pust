use rand::distributions::Standard;
use rand::prelude::*;
use rand::SeedableRng;
use rand_pcg::{Lcg128Xsl64, Pcg64};
use rand_seeder::SipHasher;

#[derive(PartialEq, Eq)]
pub struct Rnger {
    rng: Lcg128Xsl64,
}

impl Rnger {
    pub fn new(salt: String) -> Self {
        let hasher = SipHasher::from(salt);
        let mut hasher_rng = hasher.into_rng();
        let mut seed: <Pcg64 as SeedableRng>::Seed = Default::default();
        hasher_rng.fill(&mut seed);
        let rng = Pcg64::from_seed(seed);
        Rnger { rng: rng }
    }

    pub fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>,
    {
        self.rng.gen::<T>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_generate_same_output() {
        let salt_1 = String::from("test");
        let salt_2 = String::from("test");
        let mut rng_1 = Rnger::new(salt_1);
        let mut rng_2 = Rnger::new(salt_2);
        let rand_1 = rng_1.gen::<usize>();
        let rand_2 = rng_2.gen::<usize>();
        assert_eq!(rand_1, rand_2);
    }

    #[test]
    fn should_generate_different_output() {
        let salt_1 = String::from("test");
        let salt_2 = String::from("test_2");
        let mut rng_1 = Rnger::new(salt_1);
        let mut rng_2 = Rnger::new(salt_2);
        let rand_1 = rng_1.gen::<usize>();
        let rand_2 = rng_2.gen::<usize>();
        assert_ne!(rand_1, rand_2);
    }
}
