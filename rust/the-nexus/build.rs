use uuid::Uuid;

fn main() {
    // make cargo re run on every build
    println!("cargo::rerun-if-changed=INVALID_FILE");

    std::fs::write("../../static/spritesheet.ver", Uuid::new_v4().to_string())
        .expect("failed to write spritesheet ver");
}
