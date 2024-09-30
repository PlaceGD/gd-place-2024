use uuid::Uuid;

fn main() {
    // make cargo re run on every build
    println!("cargo::rerun-if-changed=INVALID_FILE");

    std::fs::write(
        "../../src/assets/spritesheet.ver",
        Uuid::new_v4().to_string(),
    )
    .expect("failed to write spritesheet ver");
}
