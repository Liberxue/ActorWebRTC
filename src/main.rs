use actix::*;
fn main() {
    let system = actix::System::new("test");

    system.run();
}