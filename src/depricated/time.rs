
pub fn elapsed_time(elapsed: u64) {
    let seconds = (elapsed % 3600) % 60;
    let minutes = (elapsed % 3600 - seconds) / 60;
    let hours = (elapsed - minutes * 60 + seconds) / 3600;
    println!("H: {}, M: {}, S: {}", hours, minutes, seconds);
}
