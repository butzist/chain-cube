pub(crate) const CHAIN: [u8; 39] = [
    4, 1, 3, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 3, 1, 2, 2, 3, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3,
    1, 3, 1, 3, 3, 3, 2,
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn total_length() {
        assert_eq!(CHAIN.iter().map(|x| *x as u32).sum::<u32>(), 4 * 4 * 4);
    }
}
