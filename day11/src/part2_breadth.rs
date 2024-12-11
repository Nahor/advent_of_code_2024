/// Same as `part2_vec` but using breadth-first instead of depth-first
use std::collections::HashMap;

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

    (0..75).for_each(|_| {
        let mut next_cache = HashMap::<u64, usize>::new();
        cache.iter().for_each(|(stone, count)| {
            map_stone(*stone)
                .into_iter()
                .for_each(|new_stone| *next_cache.entry(new_stone).or_default() += count);
        });
        cache = next_cache;
    });

    let result: usize = cache.values().sum();

    Ok(result as u64)
}

fn map_stone(stone: u64) -> Vec<u64> {
    match stone.checked_ilog10() {
        None => vec![1],
        Some(digits) if digits % 2 == 0 => {
            let pow = 10_u64.pow(digits / 2);
            let left = stone / pow;
            let right = stone - (left * pow);
            vec![left, right]
        }
        Some(_) => vec![stone * 2024],
    }
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