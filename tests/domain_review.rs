use apex_dev_formatter_meter::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 56, slack: 25, drag: 25, confidence: 51 };
    assert_eq!(review_score(case), 113);
    assert_eq!(review_lane(case), "watch");
}
