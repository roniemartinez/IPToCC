use rstest::rstest;

#[rstest]
#[case::afrinic_v4("41.0.0.1", Some("ZA"))]
#[case::afrinic_v6("2001:4200::1", Some("ZA"))]
#[case::apnic_v4("1.0.16.1", Some("JP"))]
#[case::apnic_v6("2001:200::1", Some("JP"))]
#[case::arin_v4("8.8.8.8", Some("US"))]
#[case::arin_v6("2001:4860:4860::8888", Some("US"))]
#[case::lacnic_v4("200.160.0.1", Some("BR"))]
#[case::lacnic_v6("2001:1280::1", Some("BR"))]
#[case::ripencc_v4("193.0.6.139", Some("NL"))]
#[case::ripencc_v6("2001:67c:18::1", Some("NL"))]
#[case::private_ipv4("10.0.0.0", None)]
#[case::malformed("not-an-ip", None)]
fn country_code_lookup(#[case] addr: &str, #[case] expected: Option<&str>) {
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
