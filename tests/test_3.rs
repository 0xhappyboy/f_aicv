#[cfg(test)]
mod tests {
    use f_aicv::AicvSessionReader;
    use std::path::PathBuf;
    const EXTRACT_DIR: &str = r"C:\Users\admin\Downloads\A-aicv-test\ex";
    fn open_session() -> AicvSessionReader {
        let dir = PathBuf::from(EXTRACT_DIR);
        assert!(
            dir.exists(),
            "Extract directory not found: {}",
            dir.display()
        );
        AicvSessionReader::open(&dir).expect("Failed to open session")
    }
    /// Full scan: every episode -> every version -> every scene -> every shot -> image paths
    #[test]
    fn test_full_scan_shot_images() {
        let reader = open_session();
        let mut total_shots = 0;
        let mut with_image_task = 0;
        let mut image_found = 0;
        let mut image_missing = 0;
        for ep_id in reader.episode_ids() {
            for ep_ver in reader.episode_versions(&ep_id) {
                let scenes = reader.scenes_at_version(&ep_id, ep_ver);
                for scene in scenes {
                    for sc_ver in scene.historical_versions.clone() {
                        let shots = reader.scene_shots_at_version(&ep_id, &scene.id, sc_ver);
                        for shot in shots {
                            total_shots += 1;
                            for sh_ver in shot.historical_versions.clone() {
                                let shot_data = match reader.shot_version(&shot.id, sh_ver) {
                                    Some(d) => d,
                                    None => continue,
                                };
                                if shot_data.image.is_none() {
                                    continue;
                                }
                                with_image_task += 1;
                                match reader.shot_image_path_at_version(&shot.id, sh_ver) {
                                    Some(path) => {
                                        assert!(
                                            std::path::Path::new(&path).exists(),
                                            "Image path does not exist: {}",
                                            path
                                        );
                                        println!(
                                            "[OK] ep={} v{} | scene={} v{} | shot={} v{} | image={}",
                                            ep_id, ep_ver, scene.id, sc_ver, shot.id, sh_ver, path
                                        );
                                        image_found += 1;
                                    }
                                    None => {
                                        println!(
                                            "[MISS] ep={} v{} | scene={} v{} | shot={} v{} | has image task but no file",
                                            ep_id, ep_ver, scene.id, sc_ver, shot.id, sh_ver
                                        );
                                        image_missing += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("========== SHOT IMAGE SCAN SUMMARY ==========");
        println!("Total shots scanned:       {}", total_shots);
        println!("Shots with image task:     {}", with_image_task);
        println!("Image paths resolved:      {}", image_found);
        println!("Image paths missing:       {}", image_missing);
        assert_eq!(image_missing, 0, "Some image paths could not be resolved");
    }
    /// Full scan: every shot video path
    #[test]
    fn test_full_scan_shot_videos() {
        let reader = open_session();
        let mut total_shots = 0;
        let mut with_video_task = 0;
        let mut video_found = 0;
        let mut video_missing = 0;
        for ep_id in reader.episode_ids() {
            for ep_ver in reader.episode_versions(&ep_id) {
                let scenes = reader.scenes_at_version(&ep_id, ep_ver);
                for scene in scenes {
                    for sc_ver in scene.historical_versions.clone() {
                        let shots = reader.scene_shots_at_version(&ep_id, &scene.id, sc_ver);
                        for shot in shots {
                            total_shots += 1;
                            for sh_ver in shot.historical_versions.clone() {
                                let shot_data = match reader.shot_version(&shot.id, sh_ver) {
                                    Some(d) => d,
                                    None => continue,
                                };
                                if shot_data.video.is_none() {
                                    continue;
                                }
                                with_video_task += 1;
                                match reader.shot_video_path_at_version(&shot.id, sh_ver) {
                                    Some(path) => {
                                        println!(
                                            "[OK] shot={} v{} | video={}",
                                            shot.id, sh_ver, path
                                        );
                                        video_found += 1;
                                    }
                                    None => {
                                        println!(
                                            "[MISS] shot={} v{} | has video task but no file",
                                            shot.id, sh_ver
                                        );
                                        video_missing += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("========== SHOT VIDEO SCAN SUMMARY ==========");
        println!("Total shots scanned:       {}", total_shots);
        println!("Shots with video task:     {}", with_video_task);
        println!("Video paths resolved:      {}", video_found);
        println!("Video paths missing:       {}", video_missing);
    }
    /// Full scan: every shot video frame paths
    #[test]
    fn test_full_scan_shot_video_frames() {
        let reader = open_session();
        let mut first_frame_found = 0;
        let mut last_frame_found = 0;
        let mut source_found = 0;
        for ep_id in reader.episode_ids() {
            for ep_ver in reader.episode_versions(&ep_id) {
                let scenes = reader.scenes_at_version(&ep_id, ep_ver);
                for scene in scenes {
                    for sc_ver in scene.historical_versions.clone() {
                        let shots = reader.scene_shots_at_version(&ep_id, &scene.id, sc_ver);
                        for shot in shots {
                            for sh_ver in shot.historical_versions.clone() {
                                if let Some(p) =
                                    reader.shot_video_first_frame_path_at_version(&shot.id, sh_ver)
                                {
                                    println!(
                                        "[OK] shot={} v{} | first_frame={}",
                                        shot.id, sh_ver, p
                                    );
                                    first_frame_found += 1;
                                }
                                if let Some(p) =
                                    reader.shot_video_last_frame_path_at_version(&shot.id, sh_ver)
                                {
                                    println!(
                                        "[OK] shot={} v{} | last_frame={}",
                                        shot.id, sh_ver, p
                                    );
                                    last_frame_found += 1;
                                }
                                if let Some(p) = reader.shot_video_path_at_version(&shot.id, sh_ver)
                                {
                                    source_found += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("========== VIDEO FRAME SCAN SUMMARY ==========");
        println!("First frame paths found:   {}", first_frame_found);
        println!("Last frame paths found:    {}", last_frame_found);
        println!("Source video paths found:  {}", source_found);
    }
    /// Full scan: every asset image path
    #[test]
    fn test_full_scan_asset_images() {
        let reader = open_session();
        let assets = reader.assets();
        let mut found = 0;
        let mut missing = 0;
        for asset in &assets {
            match reader.asset_image_path(&asset.id) {
                Some(path) => {
                    assert!(
                        std::path::Path::new(&path).exists(),
                        "Asset image path does not exist: {}",
                        path
                    );
                    println!(
                        "[OK] asset={} | category={:?} | name='{}' | image={}",
                        asset.id, asset.category, asset.name, path
                    );
                    found += 1;
                }
                None => {
                    println!(
                        "[MISS] asset={} | category={:?} | name='{}' | no image found",
                        asset.id, asset.category, asset.name
                    );
                    missing += 1;
                }
            }
        }
        println!("========== ASSET IMAGE SCAN SUMMARY ==========");
        println!("Total assets:              {}", assets.len());
        println!("Image paths resolved:      {}", found);
        println!("Image paths missing:       {}", missing);
    }
    /// Full scan: every asset audio path
    #[test]
    fn test_full_scan_asset_audio() {
        let reader = open_session();
        let assets = reader.assets();
        let mut found = 0;
        let mut missing = 0;
        for asset in &assets {
            match reader.asset_audio_path(&asset.id) {
                Some(path) => {
                    assert!(
                        std::path::Path::new(&path).exists(),
                        "Asset audio path does not exist: {}",
                        path
                    );
                    println!(
                        "[OK] asset={} | category={:?} | name='{}' | audio={}",
                        asset.id, asset.category, asset.name, path
                    );
                    found += 1;
                }
                None => {
                    println!(
                        "[INFO] asset={} | category={:?} | name='{}' | no audio",
                        asset.id, asset.category, asset.name
                    );
                    missing += 1;
                }
            }
        }
        println!("========== ASSET AUDIO SCAN SUMMARY ==========");
        println!("Total assets:              {}", assets.len());
        println!("Audio paths resolved:      {}", found);
        println!("Audio paths missing:       {}", missing);
    }
    /// Full scan: every asset video path
    #[test]
    fn test_full_scan_asset_videos() {
        let reader = open_session();
        let assets = reader.assets();
        let mut found = 0;
        for asset in &assets {
            if let Some(path) = reader.asset_video_path(&asset.id) {
                println!(
                    "[OK] asset={} | category={:?} | name='{}' | video={}",
                    asset.id, asset.category, asset.name, path
                );
                found += 1;
            }
        }
        println!("========== ASSET VIDEO SCAN SUMMARY ==========");
        println!("Total assets:              {}", assets.len());
        println!("Video paths resolved:      {}", found);
    }
    /// Full scan: shot all image paths across all versions
    #[test]
    fn test_full_scan_shot_all_image_paths() {
        let reader = open_session();
        let mut total_paths = 0;
        for ep_id in reader.episode_ids() {
            for ep_ver in reader.episode_versions(&ep_id) {
                let scenes = reader.scenes_at_version(&ep_id, ep_ver);
                for scene in scenes {
                    for sc_ver in scene.historical_versions.clone() {
                        let shots = reader.scene_shots_at_version(&ep_id, &scene.id, sc_ver);
                        for shot in shots {
                            let paths = reader.shot_all_image_paths(&shot.id);
                            if !paths.is_empty() {
                                println!(
                                    "shot={} | {} image path(s) across versions:",
                                    shot.id,
                                    paths.len()
                                );
                                for p in &paths {
                                    println!("  {}", p);
                                }
                                total_paths += paths.len();
                            }
                        }
                    }
                }
            }
        }
        println!("========== SHOT ALL IMAGES SCAN SUMMARY ==========");
        println!("Total image paths found:   {}", total_paths);
    }
}
