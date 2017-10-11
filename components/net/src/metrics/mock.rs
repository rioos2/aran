use rand::{thread_rng, Rng};
use std::collections::BTreeMap;

pub struct MockMetrics {}

impl MockMetrics {
    fn hs_mock() -> Option<String> {
        let mut b = BTreeMap::<u32, &'static str>::new();
        b.insert(
            1,
            r#"
    {}
    "#,
        );

        b.insert(
            2,
            r#"
    {}
    "#,
        );

        b.insert(
            3,
            r#"
    {}
    "#,
        );

        b.insert(
            4,
            r#"
    {}
    "#,
        );

        b.insert(
            5,
            r#"
    {}
    "#,
        );

        b.insert(
            6,
            r#"
    {}
    "#,
        );

        b.insert(
            7,
            r#"
    {}
    "#,
        );

        b.insert(
            8,
            r#"
    {}
    "#,
        );

        b.insert(
            9,
            r#"
    {}
    "#,
        );

        b.insert(
            10,
            r#"
    {}
    "#,
        );

        let mut rng = thread_rng();
        let mut y = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        rng.shuffle(&mut y);
        Some(b.get(&y[0]).unwrap_or(&"{}").to_string())
    }

    pub fn hs_metrics() -> Option<String> {
        Self::hs_mock()
    }
}
