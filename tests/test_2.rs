#[cfg(test)]
mod tests {
    use f_aicv::FAicv;
    use std::path::PathBuf;
    const SESSION_DIR: &str = r"C:\Users\admin\AppData\Roaming\HippoX\subsystem\StoryCanvas\StoryCanvasDialogHistory\storycanvas_session_56b8e75d-fd9c-4cdc-928a-3f80bcf2b504_1795505943265";
    const OUTPUT_DIR: &str = r"C:\Users\admin\Downloads\A-aicv-test";
    const AICV_FILE: &str = r"C:\Users\admin\Downloads\A-aicv-test\session_new_id.aicv";
    const EXTRACT_DIR: &str = r"C:\Users\admin\Downloads\A-aicv-test\extracted_new_id";
    const NEW_SESSION_ID: &str = "storycanvas_session_new_test_12345";
    /// Tests packing a real session directory into an .aicv archive for new id test.
    #[test]
    fn test_pack_for_new_id() {
        let session_dir = PathBuf::from(SESSION_DIR);
        let output_dir = PathBuf::from(OUTPUT_DIR);
        let aicv_path = PathBuf::from(AICV_FILE);
        // Verify session directory exists
        assert!(
            session_dir.exists(),
            "Session directory not found: {}",
            session_dir.display()
        );
        // Create output directory if needed
        std::fs::create_dir_all(&output_dir).unwrap();
        // Pack session
        let result = FAicv::pack(&session_dir, &aicv_path);
        assert!(result.is_ok(), "Failed to pack session: {:?}", result.err());
        // Verify .aicv file exists
        assert!(aicv_path.exists());
        let size = aicv_path.metadata().unwrap().len();
        assert!(size > 0, "Archive file is empty");
        println!("Packed session successfully for new id test:");
        println!("  Source: {}", session_dir.display());
        println!("  Output: {}", aicv_path.display());
        println!("  Size: {} bytes", size);
    }
    /// Tests unpacking an .aicv archive with a new session id.
    #[test]
    fn test_unpack_with_new_id() {
        let aicv_path = PathBuf::from(AICV_FILE);
        let extract_dir = PathBuf::from(EXTRACT_DIR);
        // Verify .aicv file exists (run test_pack_for_new_id first)
        assert!(
            aicv_path.exists(),
            "Archive file not found: {}. Run test_pack_for_new_id first.",
            aicv_path.display()
        );
        // Clean up previous extraction
        let _ = std::fs::remove_dir_all(&extract_dir);
        // Unpack archive with new session id
        let result = FAicv::unpack_with_new_id(&aicv_path, &extract_dir, NEW_SESSION_ID);
        assert!(
            result.is_ok(),
            "Failed to unpack archive with new id: {:?}",
            result.err()
        );
        // Verify extraction
        assert!(extract_dir.exists());
        let metadata = result.unwrap();
        // Verify new session id is set correctly
        assert_eq!(
            metadata.session_id, NEW_SESSION_ID,
            "Session ID was not updated correctly"
        );
        println!("Unpacked archive with new session id successfully:");
        println!("  Archive: {}", aicv_path.display());
        println!("  Output: {}", extract_dir.display());
        println!("  New Session ID: {}", metadata.session_id);
        println!("  Title: {}", metadata.title);
        println!("  Episodes: {}", metadata.episode_count());
        println!("  Total shots: {}", metadata.total_shot_count());
        println!("  Assets: {}", metadata.asset_count());
    }
    /// Tests that metadata.json contains the new session id after unpacking.
    #[test]
    fn test_metadata_json_has_new_id() {
        let extract_dir = PathBuf::from(EXTRACT_DIR);
        let metadata_path = extract_dir.join("workspace").join("metadata.json");
        // Verify extraction exists (run test_unpack_with_new_id first)
        assert!(
            metadata_path.exists(),
            "Metadata file not found: {}. Run test_unpack_with_new_id first.",
            metadata_path.display()
        );
        // Read and parse metadata.json
        let content = std::fs::read_to_string(&metadata_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        // Verify session_id field has the new id
        let session_id = json["session_id"].as_str().unwrap_or("");
        assert_eq!(
            session_id, NEW_SESSION_ID,
            "metadata.json session_id does not match new id"
        );
        println!("metadata.json session_id verified:");
        println!("  File: {}", metadata_path.display());
        println!("  Session ID: {}", session_id);
    }
    /// Tests that terminal.json contains the new session id after unpacking.
    #[test]
    fn test_terminal_json_has_new_id() {
        let extract_dir = PathBuf::from(EXTRACT_DIR);
        let terminal_path = extract_dir.join("terminal.json");
        // Verify extraction exists (run test_unpack_with_new_id first)
        assert!(
            terminal_path.exists(),
            "Terminal file not found: {}. Run test_unpack_with_new_id first.",
            terminal_path.display()
        );
        // Read and parse terminal.json
        let content = std::fs::read_to_string(&terminal_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        // Check if terminal.json is an array of tasks
        if let serde_json::Value::Array(tasks) = &json {
            // Verify each task has the new session id
            for (i, task) in tasks.iter().enumerate() {
                if let Some(session_id) = task["session_id"].as_str() {
                    assert_eq!(
                        session_id, NEW_SESSION_ID,
                        "terminal.json task {} session_id does not match new id",
                        i
                    );
                }
            }
            println!(
                "terminal.json session_id verified for {} tasks:",
                tasks.len()
            );
            println!("  File: {}", terminal_path.display());
            println!("  All tasks have Session ID: {}", NEW_SESSION_ID);
        } else {
            // If not an array, check if it's a single object
            if let Some(session_id) = json["session_id"].as_str() {
                assert_eq!(
                    session_id, NEW_SESSION_ID,
                    "terminal.json session_id does not match new id"
                );
                println!("terminal.json session_id verified:");
                println!("  File: {}", terminal_path.display());
                println!("  Session ID: {}", session_id);
            }
        }
    }
    /// Tests that config.json contains the new session id after unpacking.
    #[test]
    fn test_config_json_has_new_id() {
        let extract_dir = PathBuf::from(EXTRACT_DIR);
        let config_path = extract_dir.join("config.json");
        // Verify extraction exists (run test_unpack_with_new_id first)
        assert!(
            config_path.exists(),
            "Config file not found: {}. Run test_unpack_with_new_id first.",
            config_path.display()
        );
        // Read and parse config.json
        let content = std::fs::read_to_string(&config_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        // Verify session_id field has the new id
        let session_id = json["session_id"].as_str().unwrap_or("");
        assert_eq!(
            session_id, NEW_SESSION_ID,
            "config.json session_id does not match new id"
        );
        println!("config.json session_id verified:");
        println!("  File: {}", config_path.display());
        println!("  Session ID: {}", session_id);
    }
    /// Tests that all JSON files in the extracted directory have consistent session ids.
    #[test]
    fn test_all_json_files_consistent_id() {
        let extract_dir = PathBuf::from(EXTRACT_DIR);
        // Verify extraction exists (run test_unpack_with_new_id first)
        assert!(
            extract_dir.exists(),
            "Extract directory not found: {}. Run test_unpack_with_new_id first.",
            extract_dir.display()
        );
        let mut checked_files = 0;
        let mut files_with_session_id = 0;
        // Recursively check all JSON files
        fn check_json_files(
            dir: &std::path::Path,
            new_id: &str,
            checked: &mut usize,
            with_id: &mut usize,
        ) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        check_json_files(&path, new_id, checked, with_id);
                    } else if path.extension().map_or(false, |ext| ext == "json") {
                        *checked += 1;
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                                // Check if this JSON has a session_id field
                                if let Some(session_id) =
                                    json.get("session_id").and_then(|v| v.as_str())
                                {
                                    *with_id += 1;
                                    assert_eq!(
                                        session_id,
                                        new_id,
                                        "File {} has incorrect session_id: {}",
                                        path.display(),
                                        session_id
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        check_json_files(
            &extract_dir,
            NEW_SESSION_ID,
            &mut checked_files,
            &mut files_with_session_id,
        );
        println!("All JSON files checked for consistent session id:");
        println!("  Total JSON files: {}", checked_files);
        println!("  Files with session_id: {}", files_with_session_id);
        println!("  All session_ids match: {}", NEW_SESSION_ID);
    }
    /// Tests round-trip with new id: pack then unpack with new id.
    #[test]
    fn test_round_trip_with_new_id() {
        let session_dir = PathBuf::from(SESSION_DIR);
        let aicv_path = PathBuf::from(AICV_FILE);
        let extract_dir = PathBuf::from(EXTRACT_DIR);
        assert!(session_dir.exists());
        // Pack
        FAicv::pack(&session_dir, &aicv_path).unwrap();
        // Unpack with new id
        let _ = std::fs::remove_dir_all(&extract_dir);
        let metadata = FAicv::unpack_with_new_id(&aicv_path, &extract_dir, NEW_SESSION_ID).unwrap();
        // Verify new session id
        assert_eq!(metadata.session_id, NEW_SESSION_ID);
        // Verify key files exist
        assert!(extract_dir.join("workspace/metadata.json").exists());
        // Verify metadata.json has new id
        let metadata_content =
            std::fs::read_to_string(extract_dir.join("workspace/metadata.json")).unwrap();
        assert!(metadata_content.contains(NEW_SESSION_ID));
        println!("Round-trip with new id test passed!");
        println!("  Original: {}", session_dir.display());
        println!("  Packed: {}", aicv_path.display());
        println!("  Unpacked: {}", extract_dir.display());
        println!("  New Session ID: {}", NEW_SESSION_ID);
    }
}
