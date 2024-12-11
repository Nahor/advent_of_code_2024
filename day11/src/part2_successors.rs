/// Same as `part2_inline` but using `successors` instead of a local variable
use std::{collections::HashMap, iter::successors};

use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let stones = parse(content)?;

    // Key: stone number,
    // Value: number of time the stone exist in current blink
    let mut cache = HashMap::<u64, usize>::new();
    stones.into_iter().for_each(|stone| {
        *cache.entry(stone).or_default() += 1;
    });

    let (_, stones_final) = successors(Some((0, cache)), |(blink, cache)| {
        if *blink == 75 {
            return None;
        }
        let mut next_cache = HashMap::<u64, usize>::new();
        cache
            .iter()
            .for_each(|(stone, count)| match stone.checked_ilog10() {
                None => *next_cache.entry(1).or_default() += count,
                Some(digits) if digits % 2 == 0 => {
                    let pow = 10_u64.pow(digits / 2);
                    let left = stone / pow;
                    let right = stone - (left * pow);
                    *next_cache.entry(left).or_default() += count;
                    *next_cache.entry(right).or_default() += count
                }
                Some(_) => *next_cache.entry(stone * 2024).or_default() += count,
            });
        Some((blink + 1, next_cache))
    })
    .last()
    .unwrap();

    let result: usize = stones_final.values().sum();

    Ok(result as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"125 17"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE;

        assert_eq!(run(input).unwrap(), 65601038650482);
    }

    // #[test]
    // fn sample_sorted() {
    //     assert_eq!(
    //         run_sorted(&INPUT_SAMPLE[1..]).unwrap(),
    //         run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = read_input_u8(None).unwrap();
    //     assert_eq!(run_sorted(&input).unwrap(), run(&input).unwrap());
    // }
}
