use std::{
    thread,
    time::{self, Instant},
};

const MILLIS_IN_SEC: f64 = 1000.0;

pub fn sync(last_time: &mut Instant, frame_rate: i32) -> u128 {
    if frame_rate <= 0 {
        let now = Instant::now();
        let dt = last_time.elapsed().as_millis();
        *last_time = now;
        return dt;
    }

    let target_frame_time = time::Duration::from_millis((MILLIS_IN_SEC / frame_rate as f64) as u64);
    let elapsed_time = last_time.elapsed();

    if elapsed_time < target_frame_time {
        thread::sleep(target_frame_time - elapsed_time);
    }

    let dt = last_time.elapsed().as_millis();
    *last_time = Instant::now();
    dt
}
