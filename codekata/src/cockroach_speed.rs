pub fn speed_challenge() {
    let speed_km_per_hour = 1.08;
    let speed_cm_per_second = cockroach_speed(speed_km_per_hour);
    println!("{speed_km_per_hour}, {speed_cm_per_second}");
    let speed_cm_per_second = cockroach_speed_alternative(speed_km_per_hour);
    println!("{speed_km_per_hour}, {speed_cm_per_second}");
}

fn cockroach_speed(s: f64) -> i64 {
    (s * 100_000. / 3_600.).floor() as i64
}

fn cockroach_speed_alternative(s: f64) -> i64 {
    const KM_TO_CM: f64 = 100_000.;
    const HOUR_TO_SECOND: f64 = 1. / 3_600.;
    (s * KM_TO_CM * HOUR_TO_SECOND).floor() as i64
}
