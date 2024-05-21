#[cfg(test)]
mod tests {
    use itertools::Itertools as _;

    #[test]
    fn merge_ok1() {
        let actual = itertools::merge(vec![1, 4, 6], vec![2, 3, 5]).collect::<Vec<_>>();
        assert_eq!(actual, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_ok2() {
        let actual = vec![1, 4, 6]
            .into_iter()
            .merge(vec![2, 3, 5])
            .collect::<Vec<_>>();
        assert_eq!(actual, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn kmerge_ok1() {
        let actual = itertools::kmerge(vec![vec![1, 4, 7], vec![2, 3, 5], vec![6, 8, 9]])
            .collect::<Vec<_>>();
        assert_eq!(actual, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn kmerge_by_ok1() {
        let actual = vec![vec![9, 6, 3], vec![8, 7, 5], vec![4, 2, 1]]
            .into_iter()
            .kmerge_by(|a, b| a > b)
            .collect::<Vec<_>>();
        assert_eq!(actual, vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn chunk_by_ok1() {
        let actual = vec![1, 4, 2, 3, 5, 6]
            .into_iter()
            .chunk_by(|v| *v > 3)
            .into_iter()
            .map(|(k, g)| (k, g.collect::<Vec<_>>()))
            .collect::<Vec<(_, _)>>();
        assert_eq!(
            actual,
            vec![
                (false, vec![1]),
                (true, vec![4]),
                (false, vec![2, 3]),
                (true, vec![5, 6])
            ]
        );
    }

    #[test]
    fn chunk_by_ok2() {
        let actual = vec![1, 2, 2, 3, 3, 3]
            .into_iter()
            .chunk_by(|v| *v)
            .into_iter()
            .map(|(k, g)| (k, g.collect::<Vec<_>>()))
            .collect::<Vec<(_, _)>>();
        assert_eq!(
            actual,
            vec![(1, vec![1]), (2, vec![2, 2]), (3, vec![3, 3, 3])]
        );
    }

    #[test]
    fn cartesian_product_ok1() {
        let actual = vec![1, 2, 3]
            .into_iter()
            .cartesian_product(vec!['a', 'b'])
            .collect::<Vec<(_, _)>>();
        assert_eq!(
            actual,
            vec![(1, 'a'), (1, 'b'), (2, 'a'), (2, 'b'), (3, 'a'), (3, 'b')]
        );
    }

    #[test]
    fn multi_cartesian_product_ok1() {
        let actual = vec![vec![1, 2], vec![3, 4], vec![5, 6]]
            .into_iter()
            .multi_cartesian_product()
            .collect::<Vec<_>>();
        assert_eq!(
            actual,
            vec![
                vec![1, 3, 5],
                vec![1, 3, 6],
                vec![1, 4, 5],
                vec![1, 4, 6],
                vec![2, 3, 5],
                vec![2, 3, 6],
                vec![2, 4, 5],
                vec![2, 4, 6]
            ]
        );
    }
}
