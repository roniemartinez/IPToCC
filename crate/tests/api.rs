use core::net::{Ipv4Addr, Ipv6Addr};

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

#[rstest]
#[case::afrinic(Ipv4Addr::new(41, 58, 0, 1), Some("NG"))]
#[case::apnic(Ipv4Addr::new(118, 175, 0, 1), Some("TH"))]
#[case::arin(Ipv4Addr::new(9, 9, 9, 9), Some("US"))]
#[case::lacnic(Ipv4Addr::new(190, 191, 0, 1), Some("AR"))]
#[case::ripencc(Ipv4Addr::new(217, 0, 0, 1), Some("DE"))]
#[case::link_local(Ipv4Addr::new(169, 254, 1, 1), None)]
fn country_code_v4_typed(#[case] ip: Ipv4Addr, #[case] expected: Option<&str>) {
    assert_eq!(iptocc::country_code_v4(ip), expected);
}

#[rstest]
#[case::afrinic("2c0f::1", Some("ZA"))]
#[case::apnic("2400:3200::1", Some("CN"))]
#[case::arin("2600:1404::1", Some("US"))]
#[case::lacnic("2800:3f0:4000::1", Some("AR"))]
#[case::ripencc("2a00:1450::1", Some("IE"))]
#[case::documentation("2001:db8::1", None)]
fn country_code_v6_typed(#[case] addr: &str, #[case] expected: Option<&str>) {
    let ip: Ipv6Addr = addr.parse().unwrap();
    assert_eq!(iptocc::country_code_v6(ip), expected);
}
