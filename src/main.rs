use serde::Deserialize;

#[derive(Deserialize)]
struct BuildManifest {
    service: String,
    attempts: u8,
}

fn main() {
    let manifest: BuildManifest = serde_json::from_str(
        r#"{"service":"tsh-e2e-rust","attempts":3}"#,
    )
    .expect("fixture manifest should parse");
    assert_eq!(manifest.service, "tsh-e2e-rust");
    assert_eq!(manifest.attempts, 3);
    println!("tsh-e2e-rust-ok");
}
