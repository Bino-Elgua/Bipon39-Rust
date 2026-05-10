use std::process::Command;

use bipon39::{
    entropy_to_mnemonic, master_from_seed, mnemonic_to_seed, BiponError, DerivationMode,
};
use serde_json::{json, Map, Value};

struct VectorSpec {
    id: &'static str,
    description: &'static str,
    entropy_bytes: usize,
    passphrase: &'static str,
    include_master_keys: bool,
}

fn main() -> Result<(), BiponError> {
    let specs = [
        VectorSpec {
            id: "v001",
            description: "all-zero 128-bit entropy, no passphrase",
            entropy_bytes: 16,
            passphrase: "",
            include_master_keys: false,
        },
        VectorSpec {
            id: "v002",
            description: "all-zero 160-bit entropy, no passphrase",
            entropy_bytes: 20,
            passphrase: "",
            include_master_keys: false,
        },
        VectorSpec {
            id: "v003",
            description: "all-zero 192-bit entropy, no passphrase",
            entropy_bytes: 24,
            passphrase: "",
            include_master_keys: false,
        },
        VectorSpec {
            id: "v004",
            description: "all-zero 224-bit entropy, no passphrase",
            entropy_bytes: 28,
            passphrase: "",
            include_master_keys: false,
        },
        VectorSpec {
            id: "v005",
            description: "all-zero 256-bit entropy, no passphrase",
            entropy_bytes: 32,
            passphrase: "",
            include_master_keys: true,
        },
        VectorSpec {
            id: "v006",
            description: "all-zero 256-bit entropy, passphrase: àṣẹ (precomposed unicode)",
            entropy_bytes: 32,
            passphrase: "àṣẹ",
            include_master_keys: false,
        },
    ];

    let vectors = specs
        .iter()
        .map(generate_vector)
        .collect::<Result<Vec<_>, _>>()?;

    let output = json!({
        "schema_version": "1.1",
        "description": "BIPỌ̀N39 pinned test vectors — generated from canonical implementation",
        "generated_at": "2026-05-10",
        "generator": "cargo run --example regenerate_vectors > vectors/test_vectors.json",
        "generator_commit": current_commit(),
        "vectors": vectors,
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&output)
            .map_err(|err| BiponError::JsonParseError(err.to_string()))?
    );
    Ok(())
}

fn generate_vector(spec: &VectorSpec) -> Result<Value, BiponError> {
    let entropy = vec![0u8; spec.entropy_bytes];
    let mnemonic = entropy_to_mnemonic(&entropy)?;
    let words = mnemonic.iter().map(String::as_str).collect::<Vec<_>>();
    let seed = mnemonic_to_seed(&words, spec.passphrase)?;

    let mut vector = Map::new();
    vector.insert("id".to_string(), json!(spec.id));
    vector.insert("description".to_string(), json!(spec.description));
    vector.insert("entropy_hex".to_string(), json!(hex::encode(&entropy)));
    vector.insert("entropy_bits".to_string(), json!(spec.entropy_bytes * 8));
    vector.insert("passphrase".to_string(), json!(spec.passphrase));
    vector.insert("mnemonic".to_string(), json!(mnemonic));
    vector.insert("mnemonic_phrase".to_string(), json!(words.join(" ")));
    vector.insert("word_count".to_string(), json!(words.len()));
    vector.insert("seed_hex".to_string(), json!(hex::encode(&seed[..])));

    if spec.include_master_keys {
        let native = master_from_seed(&seed, DerivationMode::Native)?;
        let bip32 = master_from_seed(&seed, DerivationMode::Bip32)?;
        vector.insert("master_key_native_hex".to_string(), json!(native.key_hex()));
        vector.insert(
            "master_chain_native_hex".to_string(),
            json!(native.chain_code_hex()),
        );
        vector.insert("master_key_bip32_hex".to_string(), json!(bip32.key_hex()));
        vector.insert(
            "master_chain_bip32_hex".to_string(),
            json!(bip32.chain_code_hex()),
        );
    }

    Ok(Value::Object(vector))
}

fn current_commit() -> String {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|output| output.status.success().then_some(output.stdout))
        .and_then(|stdout| String::from_utf8(stdout).ok())
        .map(|commit| commit.trim().to_string())
        .filter(|commit| !commit.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}
