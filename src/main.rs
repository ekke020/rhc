mod systems;
mod core;
fn main() {
    let password_info = systems::input::take();
    systems::spawner::run_threads(password_info);
}

