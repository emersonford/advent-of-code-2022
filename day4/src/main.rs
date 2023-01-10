fn main() {
    let input = include_str!("input.txt");

    let count: u64 = input.lines().map(|line| line.split_once(",").unwrap()).map(|(fst, snd)| {
        let (fst_start, fst_end) = fst.split_once("-").map(|(fst, snd)| (fst.parse::<u64>().unwrap(), snd.parse::<u64>().unwrap())).unwrap();
        let (snd_start, snd_end) = snd.split_once("-").map(|(fst, snd)| (fst.parse::<u64>().unwrap(), snd.parse::<u64>().unwrap())).unwrap();

        if (fst_start >= snd_start && fst_end <= snd_end) || (snd_start >= fst_start && snd_end <= fst_end) {
            1
        } else {
            0
        }
    }).sum();

    println!("count: {}", count);
}
