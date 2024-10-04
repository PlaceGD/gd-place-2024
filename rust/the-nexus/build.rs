use uuid::Uuid;

fn main() {
    // make cargo re run on every build
    println!("cargo::rerun-if-changed=INVALID_FILE");

    let uuid = Uuid::new_v4().to_string();

    if !std::path::Path::new("../../static/versions").exists() {
        std::fs::create_dir_all("../../static/versions").expect("failed to create versions dir");
    }

    std::fs::write("../../static/versions/spritesheet.ver", &uuid)
        .expect("failed to write spritesheet ver");
}
