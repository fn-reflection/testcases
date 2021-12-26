#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    fn map_factory1() -> BTreeMap<i32, i32> {
        maplit::btreemap! {
          1_000_000_000 => 1,
          1_000_005_000 => 2,
          1_001_000_000 => 3,
          1_100_000_000 => 4,
        }
    }

    #[test]
    fn range_ok1() {
        let map = map_factory1();
        let actual = map.range(0..=1_001_000_000);
        assert_eq!(actual.collect::<Vec<_>>().len(), 3);
    }

    #[test]
    fn range_ok2() {
        let map = map_factory1();
        let actual = map.range(0..1_001_000_000);
        assert_eq!(actual.collect::<Vec<_>>().len(), 2);
    }
}
