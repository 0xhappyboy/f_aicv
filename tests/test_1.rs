#[cfg(test)]
mod tests {
    use f_aicv::FAicv;
    use std::path::PathBuf;
    const SESSION_DIR: &str = r"C:\Users\admin\AppData\Roaming\HippoX\subsystem\StoryCanvas\StoryCanvasDialogHistory\storycanvas_session_1795461013178";
    const OUTPUT_DIR: &str = r"C:\Users\admin\Downloads\A-aicv-test";
    const AICV_FILE: &str = r"C:\Users\admin\Downloads\A-aicv-test\session.aicv";
    const EXTRACT_DIR: &str = r"C:\Users\admin\Downloads\A-aicv-test\extracted";
    /// Tests packing a real session directory into an .aicv archive.
    #[test]
    fn test_pack_real_session() {
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
        println!("Packed session successfully:");
        println!("  Source: {}", session_dir.display());
        println!("  Output: {}", aicv_path.display());
        println!("  Size: {} bytes", size);
    }
    /// Tests unpacking an .aicv archive to a directory.
    #[test]
    fn test_unpack_archive() {
        let aicv_path = PathBuf::from(AICV_FILE);
        let extract_dir = PathBuf::from(EXTRACT_DIR);
        // Verify .aicv file exists (run test_pack_real_session first)
        assert!(
            aicv_path.exists(),
            "Archive file not found: {}. Run test_pack_real_session first.",
            aicv_path.display()
        );
        // Clean up previous extraction
        let _ = std::fs::remove_dir_all(&extract_dir);
        // Unpack archive
        let result = FAicv::unpack(&aicv_path, &extract_dir);
        assert!(
            result.is_ok(),
            "Failed to unpack archive: {:?}",
            result.err()
        );
        // Verify extraction
        assert!(extract_dir.exists());
        let metadata = result.unwrap();
        println!("Unpacked archive successfully:");
        println!("  Archive: {}", aicv_path.display());
        println!("  Output: {}", extract_dir.display());
        println!("  Session ID: {}", metadata.session_id);
        println!("  Title: {}", metadata.title);
        println!("  Episodes: {}", metadata.episode_count());
        println!("  Total shots: {}", metadata.total_shot_count());
        println!("  Assets: {}", metadata.asset_count());
    }
    /// Tests reading metadata from .aicv without unpacking.
    #[test]
    fn test_read_metadata() {
        let aicv_path = PathBuf::from(AICV_FILE);
        assert!(
            aicv_path.exists(),
            "Archive file not found: {}",
            aicv_path.display()
        );
        let result = FAicv::read_metadata(&aicv_path);
        assert!(
            result.is_ok(),
            "Failed to read metadata: {:?}",
            result.err()
        );
        let metadata = result.unwrap();
        println!("Metadata read successfully:");
        println!("  Session ID: {}", metadata.session_id);
        println!("  Title: {}", metadata.title);
        println!("  Description: {}", metadata.description);
        println!("  Created: {}", metadata.created_at);
    }
    /// Tests archive summary.
    #[test]
    fn test_archive_summary() {
        let aicv_path = PathBuf::from(AICV_FILE);
        assert!(
            aicv_path.exists(),
            "Archive file not found: {}",
            aicv_path.display()
        );
        let summary = FAicv::summary(&aicv_path).unwrap();
        println!("Archive Summary:");
        println!("  Session ID: {}", summary.session_id);
        println!("  Version: {}", summary.version);
        println!("  Total entries: {}", summary.total_entries);
        println!(
            "  Total size: {} bytes ({:.2} MB)",
            summary.total_size_bytes,
            summary.total_size_bytes as f64 / 1024.0 / 1024.0
        );
        println!("  Images: {}", summary.image_count);
        println!("  Videos: {}", summary.video_count);
        println!("  Audio: {}", summary.audio_count);
        println!("  Metadata files: {}", summary.metadata_count);
    }
    /// Tests archive validation.
    #[test]
    fn test_validate_archive() {
        let aicv_path = PathBuf::from(AICV_FILE);
        assert!(
            aicv_path.exists(),
            "Archive file not found: {}",
            aicv_path.display()
        );
        let is_valid = FAicv::validate(&aicv_path).unwrap();
        assert!(is_valid, "Archive validation failed");
        println!("Archive is valid: {}", aicv_path.display());
    }
    /// Tests round-trip: pack then unpack.
    #[test]
    fn test_round_trip() {
        let session_dir = PathBuf::from(SESSION_DIR);
        let aicv_path = PathBuf::from(AICV_FILE);
        let extract_dir = PathBuf::from(EXTRACT_DIR);
        assert!(session_dir.exists());
        // Pack
        FAicv::pack(&session_dir, &aicv_path).unwrap();
        // Unpack
        let _ = std::fs::remove_dir_all(&extract_dir);
        FAicv::unpack(&aicv_path, &extract_dir).unwrap();
        // Verify key files exist
        assert!(extract_dir.join("workspace/metadata.json").exists());
        println!("Round-trip test passed!");
        println!("  Original: {}", session_dir.display());
        println!("  Packed: {}", aicv_path.display());
        println!("  Unpacked: {}", extract_dir.display());
    }
}
