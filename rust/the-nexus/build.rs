use uuid::Uuid;

fn main() {
    // make cargo re run on every build
    println!("cargo::rerun-if-changed=INVALID_FILE");

    let uuid = Uuid::new_v4().to_string();

    std::fs::write("../../static/versions/spritesheet.ver", &uuid)
        .expect("failed to write spritesheet ver");
}
