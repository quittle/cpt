use std::collections::HashSet;

use rand::prelude::*;

pub trait RandomProvider {
    // Returns a random value between lower and upper bound, inclusive.
    fn pick_linear_u64(&self, lower_bound: u64, upper_bound: u64) -> u64;
    // Returns a random value between lower and upper bound, inclusive.
    fn pick_linear_usize(&self, lower_bound: usize, upper_bound: usize) -> usize;
}

pub trait RandomPicker<T> {
    fn pick_linear(&self, random_provider: &dyn RandomProvider) -> Option<&T>;
    fn pick_n_unique_linear(&self, count: usize, random_provider: &dyn RandomProvider) -> Vec<&T>;
}

impl<T> RandomPicker<T> for Vec<T> {
    fn pick_linear(&self, random_provider: &dyn RandomProvider) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.get(random_provider.pick_linear_usize(0, self.len() - 1))
        }
    }

    fn pick_n_unique_linear(&self, count: usize, random_provider: &dyn RandomProvider) -> Vec<&T> {
        let mut result = Vec::new();
        let goal_count = count.min(self.len());
        result.reserve(goal_count);

        let mut picked_incides = HashSet::new();
        while result.len() < goal_count {
            let index = random_provider.pick_linear_usize(0, self.len() - 1);
            if picked_incides.insert(index) {
                result.push(&self[index]);
            }
        }
        result
    }
}

#[macro_export]
macro_rules! count {
    () => { 0usize };
    ($e:expr) => { 1usize };
    ($_head:expr, $($tail:expr),*) => {1usize + count!($($tail),*)};
}

#[macro_export]
macro_rules! random_choice {
    ($random_provider:expr, $($x:expr),+ ) => {
        {
            use std::borrow::Borrow;
            let random_provider: &dyn RandomProvider = $random_provider.borrow();
            let option: usize = random_provider.pick_linear_usize(0, count!($($x),*) - 1);

            // To avoid evaluating the choices that aren't picked, we generate a giant if statement
            // that has a side-effect for each branch. Other options do not work
            // 1. A simpler set of if statement. The final expression must evaluate to the right
            //    type. Individual if statements will not produce the needed final statement.
            // 2. match + case statements. Rust macros cannot individual generate match arms as they
            //    are not standalone AST nodes. The index could theoretically be passed recursively
            //    to the macro but because it would require a unique invocation per index, we're out
            //    of luck.
            let mut i: usize = 0;
            if false {
                unreachable!()
            }
            $(
                else if (i+=1) == () && option == i - 1  {
                    $x
                }
            )+
            else {
                unreachable!()
            }
        }
    };
}

#[derive(Default)]
pub struct DefaultRandomProvider {}

impl RandomProvider for DefaultRandomProvider {
    fn pick_linear_u64(&self, lower_bound: u64, upper_bound: u64) -> u64 {
        thread_rng().gen_range(lower_bound..=upper_bound)
    }

    fn pick_linear_usize(&self, lower_bound: usize, upper_bound: usize) -> usize {
        thread_rng().gen_range(lower_bound..=upper_bound)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{DefaultRandomProvider, RandomPicker, RandomProvider};

    #[test]
    fn test_pick_linear() {
        let random = DefaultRandomProvider::default();
        for _ in 0..1000 {
            let choice_u64 = random.pick_linear_u64(10, 20);
            assert!((10..=20).contains(&choice_u64));
            let choice_usize = random.pick_linear_usize(10, 20);
            assert!((10..=20).contains(&choice_usize));
        }
    }

    #[test]
    fn test_pick_choices() {
        let random = DefaultRandomProvider::default();

        for _ in 0..1000 {
            let mut call_count = 0;
            let mut expensive = |value: i32| -> i32 {
                call_count += 1;
                value
            };

            let option = random_choice!(random, expensive(1), expensive(2), expensive(3));
            // let option = random_choice2!(random, expensive(1), expensive(2), expensive(3));

            assert_eq!(call_count, 1, "Only the chosen option should be evaluated");
            assert!([1, 2, 3].contains(&option));
        }
    }

    #[test]
    fn test_pick_n_unique_linear() {
        let random = DefaultRandomProvider::default();

        let values: Vec<u8> = (1..=10).collect();

        for _ in 0..1000 {
            assert_eq!(values.pick_n_unique_linear(0, &random).len(), 0);

            {
                let result = values.pick_n_unique_linear(1, &random);
                assert_eq!(result.len(), 1);
                assert!(*result[0] >= 1 && *result[0] <= 10);
            }

            {
                let result = values.pick_n_unique_linear(100, &random);
                assert_eq!(result.len(), 10);
                let result_set: HashSet<u8> = result.iter().map(|v| **v).collect();
                assert_eq!(result_set.len(), 10);
            }
        }
    }
}
