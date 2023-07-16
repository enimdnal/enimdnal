use nanorand::{RandomRange, Rng};

pub trait IteratorRandom: Iterator + Sized {
    fn choose_multiple<const OUT: usize, R: Rng<OUT> + ?Sized>(
        mut self,
        rng: &mut R,
        n: usize,
    ) -> Vec<Self::Item> {
        let mut reservoir = Vec::with_capacity(n);
        reservoir.extend(self.by_ref().take(n));

        if reservoir.len() < n {
            reservoir.shrink_to_fit();
            return reservoir;
        }

        for (i, elem) in self.enumerate() {
            let upper_bound = n + i + 1;
            let index = RandomRange::random_range(rng, 0..upper_bound);

            if let Some(slot) = reservoir.get_mut(index) {
                *slot = elem;
            }
        }

        reservoir
    }
}

impl<I: Iterator + Sized> IteratorRandom for I {}
