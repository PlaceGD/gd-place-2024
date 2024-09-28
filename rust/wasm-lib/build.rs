use uuid::Uuid;

fn main() {
    std::fs::write("../../src/assets/wasm.ver", Uuid::new_v4().to_string())
        .expect("failed to write wasm ver");
}
