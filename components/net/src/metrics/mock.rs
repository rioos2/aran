use rand::{thread_rng, Rng};
use std::collections::BTreeMap;
pub struct MockMetrics {}

impl MockMetrics {
    fn hs_mock() -> Option<String> {
        let mut b = BTreeMap::<u32, &'static str>::new();
        b.insert(
            1,
            r#"
            {"metrics": {
              "title": "Assembly metric",
              "from_date": "2001-01-11:10:1010Z",
              "to_date": "2011-01-11:10:1010Z",
              "item": [{
          "Id": "asm001",
                "name": "cpu",
                "values": [
                {  "date":"Fri Jan 01 2017 00:00:00 GMT+0400",
        "value":"20" },
        {
              "date": "Fri Jan 02 2017 00:00:00 GMT+0400 ",
              "value": "10"
            }, {
              "date": "Fri Jan 03 2017 00:00:00 GMT+0400 ",
              "value": "15"
            }, {
              "date": "Fri Jan 04 2017 00:00:00 GMT+0400 ",
              "value": "14"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "01"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "55"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "66"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "76"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "23"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "42"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "18"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "45"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "60"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "70"
            }

                        ]
            }
          ]

        }
        }
    "#,
        );

        b.insert(
            2,
            r#"
            {"metrics": {
              "title": "Assembly metric",
              "from_date": "2001-01-11:16:1010Z",
              "to_date": "2011-01-11:10:1010Z",
              "item": [{
          "Id": "asm001",
                "name": "cpu",
                "values": [

        {  "date":"Fri Jan 01 2017 00:00:00 GMT+0400",
        "value":"80" },
        {
        "date": "Fri Jan 02 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 03 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 04 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }

                        ]
            }
          ]

        }
        }

    "#,
        );

        b.insert(
            3,
            r#"
            {"metrics": {
              "title": "Assembly metric",
              "from_date": "2001-01-11:10:1010Z",
              "to_date": "2011-01-11:10:1010Z",
              "item": [{
          "Id": "asm001",
                "name": "cpu",
                "values": [
                {  "date":"Fri Jan 01 2017 00:00:00 GMT+0400",
        "value":"20" },
        {
              "date": "Fri Jan 02 2017 00:00:00 GMT+0400 ",
              "value": "45"
            }, {
              "date": "Fri Jan 03 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 04 2017 00:00:00 GMT+0400 ",
              "value": "14"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "01"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "55"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "66"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "76"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "23"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "42"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "18"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "45"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "60"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "70"
            }

                        ]
            }
          ]

        }
        }
    "#,
        );

        b.insert(
            4,
            r#"
            {"metrics": {
              "title": "Assembly metric",
              "from_date": "2001-01-11:10:1010Z",
              "to_date": "2011-01-11:10:1010Z",
              "item": [{
          "Id": "asm001",
                "name": "cpu",
                "values": [
                {  "date":"Fri Jan 01 2017 00:00:00 GMT+0400",
        "value":"25" },
        {
              "date": "Fri Jan 02 2017 00:00:00 GMT+0400 ",
              "value": "03"
            }, {
              "date": "Fri Jan 03 2017 00:00:00 GMT+0400 ",
              "value": "05"
            }, {
              "date": "Fri Jan 04 2017 00:00:00 GMT+0400 ",
              "value": "14"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "01"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "77"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "66"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "22"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "33"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "04"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "07"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "12"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }

                        ]
            }
          ]

        }
        }
    "#,
        );

        b.insert(
            5,
            r#"
            {"metrics": {
              "title": "Assembly metric",
              "from_date": "2001-01-11:10:1010Z",
              "to_date": "2011-01-11:10:1010Z",
              "item": [{
          "Id": "asm001",
                "name": "cpu",
                "values": [
                {  "date":"Fri Jan 01 2017 00:00:00 GMT+0400",
        "value":"25" },
        {
              "date": "Fri Jan 02 2017 00:00:00 GMT+0400 ",
              "value": "03"
            }, {
              "date": "Fri Jan 03 2017 00:00:00 GMT+0400 ",
              "value": "05"
            }, {
              "date": "Fri Jan 04 2017 00:00:00 GMT+0400 ",
              "value": "14"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "01"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "77"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "66"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "22"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "33"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "04"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "07"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "12"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }

                        ]
            }
          ]

        }
        }
    "#,
        );

        b.insert(
            6,
            r#"
            {"metrics": {
              "title": "Assembly metric",
              "from_date": "2001-01-11:10:1010Z",
              "to_date": "2011-01-11:10:1010Z",
              "item": [{
          "Id": "asm001",
                "name": "cpu",
                "values": [
                {  "date":"Fri Jan 01 2017 00:00:00 GMT+0400",
        "value":"20" },
        {
              "date": "Fri Jan 02 2017 00:00:00 GMT+0400 ",
              "value": "10"
            }, {
              "date": "Fri Jan 03 2017 00:00:00 GMT+0400 ",
              "value": "15"
            }, {
              "date": "Fri Jan 04 2017 00:00:00 GMT+0400 ",
              "value": "14"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "01"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "55"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "66"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "76"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "23"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "42"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "18"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "45"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "60"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "70"
            }

                        ]
            }
          ]

        }
        }
    "#,
        );

        b.insert(
            7,
            r#"
            {"metrics": {
              "title": "Assembly metric",
              "from_date": "2001-01-11:16:1010Z",
              "to_date": "2011-01-11:10:1010Z",
              "item": [{
          "Id": "asm001",
                "name": "cpu",
                "values": [

        {  "date":"Fri Jan 01 2017 00:00:00 GMT+0400",
        "value":"80" },
        {
        "date": "Fri Jan 02 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 03 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 04 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }, {
        "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
        "value": "80"
        }

                        ]
            }
          ]

        }
        }

    "#,
        );

        b.insert(
            8,
            r#"
            {"metrics": {
              "title": "Assembly metric",
              "from_date": "2001-01-11:10:1010Z",
              "to_date": "2011-01-11:10:1010Z",
              "item": [{
          "Id": "asm001",
                "name": "cpu",
                "values": [
                {  "date":"Fri Jan 01 2017 00:00:00 GMT+0400",
        "value":"20" },
        {
              "date": "Fri Jan 02 2017 00:00:00 GMT+0400 ",
              "value": "45"
            }, {
              "date": "Fri Jan 03 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 04 2017 00:00:00 GMT+0400 ",
              "value": "14"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "01"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "55"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "66"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "76"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "23"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "42"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "18"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "45"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "60"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "70"
            }

                        ]
            }
          ]

        }
        }
    "#,
        );

        b.insert(
            9,
            r#"
            {"metrics": {
              "title": "Assembly metric",
              "from_date": "2001-01-11:10:1010Z",
              "to_date": "2011-01-11:10:1010Z",
              "item": [{
          "Id": "asm001",
                "name": "cpu",
                "values": [
                {  "date":"Fri Jan 01 2017 00:00:00 GMT+0400",
        "value":"25" },
        {
              "date": "Fri Jan 02 2017 00:00:00 GMT+0400 ",
              "value": "03"
            }, {
              "date": "Fri Jan 03 2017 00:00:00 GMT+0400 ",
              "value": "05"
            }, {
              "date": "Fri Jan 04 2017 00:00:00 GMT+0400 ",
              "value": "14"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "01"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "77"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "66"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "22"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "33"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "04"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "07"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "12"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }

                        ]
            }
          ]

        }
        }
    "#,
        );

        b.insert(
            10,
            r#"
            {"metrics": {
              "title": "Assembly metric",
              "from_date": "2001-01-11:10:1010Z",
              "to_date": "2011-01-11:10:1010Z",
              "item": [{
          "Id": "asm001",
                "name": "cpu",
                "values": [
                {  "date":"Fri Jan 01 2017 00:00:00 GMT+0400",
        "value":"25" },
        {
              "date": "Fri Jan 02 2017 00:00:00 GMT+0400 ",
              "value": "03"
            }, {
              "date": "Fri Jan 03 2017 00:00:00 GMT+0400 ",
              "value": "05"
            }, {
              "date": "Fri Jan 04 2017 00:00:00 GMT+0400 ",
              "value": "14"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "01"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "17"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "77"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "66"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "22"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "33"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "04"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "07"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "12"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "11"
            }, {
              "date": "Fri Jan 05 2017 00:00:00 GMT+0400 ",
              "value": "13"
            }

                        ]
            }
          ]

        }
        }
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
