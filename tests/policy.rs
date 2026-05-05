use apex_dev_formatter_meter::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 85, capacity: 93, latency: 14, risk: 15, weight: 10 };
    assert_eq!(score(signal), 132);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 78, capacity: 71, latency: 24, risk: 22, weight: 9 };
    assert_eq!(score(signal), 4);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 82, capacity: 104, latency: 27, risk: 23, weight: 5 };
    assert_eq!(score(signal), 14);
    assert_eq!(classify(signal), "review");
}
