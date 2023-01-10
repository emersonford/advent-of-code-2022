fn main() {
    let input = include_str!("input.txt");

    let ranges = input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(fst, snd)| {
            let fst_range = fst
                .split_once("-")
                .map(|(fst, snd)| (fst.parse::<u64>().unwrap(), snd.parse::<u64>().unwrap()))
                .unwrap();
            let snd_range = snd
                .split_once("-")
                .map(|(fst, snd)| (fst.parse::<u64>().unwrap(), snd.parse::<u64>().unwrap()))
                .unwrap();

            (fst_range, snd_range)
        })
        .collect::<Vec<_>>();

    let part1_count: u64 = ranges
        .iter()
        .map(|((fst_start, fst_end), (snd_start, snd_end))| {
            // FST_START .. SND_START .. SND_END .. FST_END
            // SND_START .. FST_START .. FST_END .. SND_END
            if (fst_start >= snd_start && fst_end <= snd_end)
                || (snd_start >= fst_start && snd_end <= fst_end)
            {
                1
            } else {
                0
            }
        })
        .sum();

    println!("part 1: {}", part1_count);

    let part2_count: u64 = ranges
        .iter()
        .map(|((fst_start, fst_end), (snd_start, snd_end))| {
            // FST_START .. SND_START .. FST_END .. SND_END
            // SND_START .. FST_START .. SND_END .. FST_END
            // FST_START .. SND_START .. SND_END .. FST_END
            // SND_START .. FST_START .. FST_END .. SND_END
            if (fst_start >= snd_start && fst_start <= snd_end)
                || (fst_end >= snd_start && fst_end <= snd_end)
                || (snd_start >= fst_start && snd_start <= fst_end)
            {
                1
            } else {
                0
            }
        })
        .sum();

    println!("part 2: {}", part2_count);
}
