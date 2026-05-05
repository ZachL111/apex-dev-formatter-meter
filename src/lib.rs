#[derive(Clone, Copy, Debug)]
pub struct Signal {
    pub demand: i32,
    pub capacity: i32,
    pub latency: i32,
    pub risk: i32,
    pub weight: i32,
}

pub const THRESHOLD: i32 = 156;
const RISK_PENALTY: i32 = 7;
const LATENCY_PENALTY: i32 = 4;
const WEIGHT_BONUS: i32 = 3;

pub fn score(signal: Signal) -> i32 {
    signal.demand * 2 + signal.capacity + signal.weight * WEIGHT_BONUS
        - signal.latency * LATENCY_PENALTY
        - signal.risk * RISK_PENALTY
}

pub fn classify(signal: Signal) -> &'static str {
    if score(signal) >= THRESHOLD { "accept" } else { "review" }
}
