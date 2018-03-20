// Copyright 2018 The Rio Advancement Inc

use router::Router;
use serde_json;
use self::*;

#[test]
fn test_json_response_for_simple_val() {
    let str_val = "sghdkgskgskldghshgsd";
    struct SampleAPI;
    impl Api for SampleAPI {
        fn wire<'b>(&self, _: &'b mut Router) {
            return;
        }
    }
    let stub = SampleAPI;
    let result = stub.ok_response(&serde_json::to_value(str_val).unwrap());
    assert!(result.is_ok());
}
