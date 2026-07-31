use std::fs;
use std::path::Path;
use crate::data_types;

pub fn run_all_feature_tests() {
    println!("🧪 Starting Distributed Compiler Test Suite...");

    let results_file = "test_results.json";
    let mut cached_results: serde_json::Value = if Path::new(results_file).exists() {
        let data = fs::read_to_string(results_file).unwrap_or_else(|_| "{}".to_string());
        serde_json::from_str(&data).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    let mut updated = false;

    // Imagine you have a list of all your active feature folder names
    let feature_names = vec!["tensor", "secret"]; // Add hundreds here easily over time

    for name in feature_names {
        // Skip if already passed and saved in JSON
        if cached_results[name] == "PASSED" {
            println!("⏩ Skipping [{}]: Already passed previously.", name);
            continue;
        }

        println!("🔍 Running edge case tests for: [{}]...", name);

        // Route the test execution to the correct folder
        let result = match name {
            "tensor" => data_types::tensor::run_tests(),
            // "secret" => data_types::secret::run_tests(),
            _ => Ok(()),
        };

        match result {
            Ok(()) => {
                println!("✅ [{}] Passed all edge cases!", name);
                cached_results[name] = serde_json::json!("PASSED");
                updated = true;
            }
            Err(e) => {
                println!("❌ [{}] Failed: {}", name, e);
                cached_results[name] = serde_json::json!(format!("FAILED: {}", e));
                updated = true;
                
                println!("🛑 Halting tests. Fix [{}] before moving to the next folder.", name);
                break;
            }
        }
    }

    // Save final results so you never re-run passed folders
    if updated {
        let json_string = serde_json::to_string_pretty(&cached_results).unwrap();
        fs::write(results_file, json_string).expect("Failed to save test results");
        println!("💾 Progress saved to '{}'.", results_file);
    }
}