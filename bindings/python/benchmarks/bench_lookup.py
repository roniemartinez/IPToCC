import pytest

V4_CASES = [
    ("afrinic", "41.0.0.1", "ZA"),
    ("apnic", "1.0.16.1", "JP"),
    ("arin", "8.8.8.8", "US"),
    ("lacnic", "200.160.0.1", "BR"),
    ("ripencc", "193.0.6.139", "NL"),
    ("miss_private", "10.0.0.0", None),
]

V6_CASES = [
    ("afrinic", "2001:4200::1", "ZA"),
    ("apnic", "2001:200::1", "JP"),
    ("arin", "2001:4860:4860::8888", "US"),
    ("lacnic", "2001:1280::1", "BR"),
    ("ripencc", "2001:67c:18::1", "NL"),
    ("miss_loopback", "::1", None),
]

BATCH_SIZES = [10, 100, 1000, 10000]


@pytest.fixture(scope="session")
def ours():
    import iptocc

    return iptocc.country_code


@pytest.mark.parametrize(("name", "addr", "expected"), V4_CASES, ids=[c[0] for c in V4_CASES])
def test_iptocc_v4(benchmark, ours, name, addr, expected):
    benchmark.group = "v4_single"
    result = benchmark(ours, addr)
    assert result == expected


@pytest.mark.parametrize(("name", "addr", "expected"), V6_CASES, ids=[c[0] for c in V6_CASES])
def test_iptocc_v6(benchmark, ours, name, addr, expected):
    benchmark.group = "v6_single"
    result = benchmark(ours, addr)
    assert result == expected


@pytest.mark.parametrize("size", BATCH_SIZES, ids=[f"n={n}" for n in BATCH_SIZES])
def test_iptocc_batch(benchmark, ours, size):
    benchmark.group = "batch"
    addrs = [c[1] for c in V4_CASES] * (size // len(V4_CASES) + 1)
    addrs = addrs[:size]
    result = benchmark(ours, addrs)
    assert len(result) == size
