pub mod trait_fromstr_unstable {

    use std::{str::FromStr, string::ParseError};

    pub struct Elves {
        calories: Vec<u32>,
    }

    impl Elves {
        pub fn get_fat(&self) -> &u32 {
            &self.calories[0]
        }

        pub fn get_fat_sum(&self, top: usize) -> u32 {
            let mut sum = 0;
            for i in 0..top {
                sum += self.calories[i];
            }
            sum
        }
    }

    impl FromStr for Elves {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut calories = s
                .trim()
                .split("\n\n")
                .map(|elf| {
                    elf.split("\n")
                        .fold(0, |acc, itm| acc + itm.parse::<u32>().unwrap())
                })
                .collect::<Vec<u32>>();
            calories.sort_unstable();
            calories.reverse();
            Ok(Elves { calories })
        }
    }
}

pub mod trait_fromstr {
    use std::{str::FromStr, string::ParseError};

    pub struct Elves {
        calories: Vec<u32>,
    }

    impl Elves {
        pub fn get_fat(&self) -> &u32 {
            &self.calories[0]
        }

        pub fn get_fat_sum(&self, top: usize) -> u32 {
            let mut sum = 0;
            for i in 0..top {
                sum += self.calories[i];
            }
            sum
        }
    }

    impl FromStr for Elves {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut calories = s
                .trim()
                .split("\n\n")
                .map(|elf| {
                    elf.split("\n")
                        .fold(0, |acc, itm| acc + itm.parse::<u32>().unwrap())
                })
                .collect::<Vec<u32>>();
            calories.sort();
            calories.reverse();
            Ok(Elves { calories })
        }
    }
}

pub mod sorting_trait {
    trait Top<T> {
        fn top<const N: usize>(self) -> [T; N];
    }

    impl<T: Default + PartialOrd, U: Iterator<Item = T>> Top<T> for U {
        fn top<const N: usize>(self) -> [T; N] {
            let mut top = core::array::from_fn(|_| Default::default());
            for value in self {
                for i in 0..N {
                    if let Some(top_value) = top.get(i) {
                        if value > *top_value {
                            top[i..].rotate_right(1);
                            top[i] = value;
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            top
        }
    }

    pub fn combinator(input: &str) -> u64 {
        input
            .split("\n\n")
            .map(|batch| {
                batch
                    .lines()
                    .map(|line| line.parse::<u64>().unwrap())
                    .sum::<u64>()
            })
            .top::<3>()
            .iter()
            .sum()
    }
}

pub fn sort_arr<T: Ord>(arr: &mut [T]) {
    sorting::bubble_sort(arr);
}

mod sorting {
    pub fn selection_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            let mut min_idx = i;
            for j in (i + 1)..len {
                if arr[j] < arr[min_idx] {
                    min_idx = j;
                }
            }
            arr.swap(min_idx, i);
        }
    }

    pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_bubble_sort() {
            let mut arr = [6, 2, 4, 1, 9, -2, 5];
            bubble_sort(&mut arr);
            assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
        }

        #[test]
        fn test_selection_sort() {
            let mut arr = [6, 2, 4, 1, 9, -2, 5];
            selection_sort(&mut arr);
            assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
        }
    }
}
