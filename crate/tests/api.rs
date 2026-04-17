use test_case::test_case;

#[test_case("41.0.0.1",               Some("ZA") ; "afrinic_v4")]
#[test_case("2001:4200::1",           Some("ZA") ; "afrinic_v6")]
#[test_case("1.0.16.1",               Some("JP") ; "apnic_v4")]
#[test_case("2001:200::1",            Some("JP") ; "apnic_v6")]
#[test_case("8.8.8.8",                Some("US") ; "arin_v4")]
#[test_case("2001:4860:4860::8888",   Some("US") ; "arin_v6")]
#[test_case("200.160.0.1",            Some("BR") ; "lacnic_v4")]
#[test_case("2001:1280::1",           Some("BR") ; "lacnic_v6")]
#[test_case("193.0.6.139",            Some("NL") ; "ripencc_v4")]
#[test_case("2001:67c:18::1",         Some("NL") ; "ripencc_v6")]
#[test_case("10.0.0.0",               None       ; "private_ipv4")]
#[test_case("not-an-ip",              None       ; "malformed")]
fn country_code_lookup(addr: &str, expected: Option<&str>) {
    assert_eq!(iptocc::country_code(addr), expected);
}

#[test]
fn batch_lookup() {
    let addrs = ["8.8.8.8", "1.0.16.1", "2001:4860:4860::8888", "10.0.0.0", "not-an-ip"];
    let results = iptocc::country_codes(addrs);
    assert_eq!(results, vec![Some("US"), Some("JP"), Some("US"), None, None]);
}

#[test]
fn batch_empty_input() {
    let empty: [&str; 0] = [];
    assert!(iptocc::country_codes(empty).is_empty());
}

#[test]
fn batch_accepts_vec_of_string() {
    let addrs: Vec<String> = vec!["8.8.8.8".to_string(), "10.0.0.0".to_string()];
    let results = iptocc::country_codes(&addrs);
    assert_eq!(results, vec![Some("US"), None]);
}
