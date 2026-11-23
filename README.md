<h1 align="center">
    f_aicv
</h1>
<h4 align="center">
A Rust-based engine for the novel AI Video Infinite Canvas Vector (*.aicv) interchange protocol. It defines the data abstraction layer for unbounded canvas graphs, shot linkage and bidirectional NLE communication, and provides complete serialization, deserialization and structured manipulation capabilities for AI drama production pipelines.
</h4>
<p align="center">
  <a href="https://github.com/0xhappyboy/f_aicv/LICENSE"><img src="https://img.shields.io/badge/Apache-2.0-d1d1f6.svg?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=googledocs&label=license&logoColor=BEC5C9" alt="License"></a>
</p>
<p align="center">
<a href="./README_zh-CN.md">简体中文</a> | <a href="./README.md">English</a>
</p>

## Example

```rust
// Pack a session directory into a .aicv archive
FAicv::pack("path/to/session_dir", "output.aicv")?;
// Unpack a .aicv archive into a directory
FAicv::unpack("output.aicv", "restored_session")?;
// Read metadata without unpacking
let metadata = FAicv::read_metadata("output.aicv")?;
// Get archive summary
let summary = FAicv::summary("output.aicv")?;
// Validate archive file
let is_valid = FAicv::validate("output.aicv")?;
```

## Archive Structure (.aicv unpacked)

```text
session_dir/
├── workspace/
│   ├── metadata.json
│   ├── shots/
│   │   └── <shot_id>/
│   │       └── e<episode_version>/
│   │           └── s<scene_version>/
│   │               └── v<shot_version>/
│   │                   ├── image_1.png
│   │                   ├── image_2.png
│   │                   └── tasks/
│   │                       ├── image/
│   │                       │   └── <task_id>.json
│   │                       └── video/
│   │                           └── <task_id>/
│   │                               ├── metadata.json
│   │                               ├── source.mp4
│   │                               ├── first_frame.png
│   │                               └── last_frame.png
│   └── assets/
│       └── <asset_id>/
│           ├── character.png
│           ├── prop.png
│           ├── environment.png
│           ├── atmosphere.png
│           ├── lighting.png
│           └── products.mp3
└── [other preserved files]
```

### Directory Explanation

| Path                                                                             | Description                                                             |
| -------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| `session_dir/`                                                                   | Root directory of the AI Canvas session                                 |
| `workspace/`                                                                     | Core workspace containing all session data                              |
| `workspace/metadata.json`                                                        | Main metadata file containing episodes, scenes, shots, assets structure |
| `workspace/shots/`                                                               | Directory containing all shot media files                               |
| `workspace/shots/<shot_id>/`                                                     | Individual shot directory identified by UUID                            |
| `workspace/shots/<shot_id>/e<episode_version>/`                                  | Episode version directory (e.g., e1, e2, e3)                            |
| `workspace/shots/<shot_id>/e<episode_version>/s<scene_version>/`                 | Scene version directory (e.g., s1, s2)                                  |
| `workspace/shots/<shot_id>/e<episode_version>/s<scene_version>/v<shot_version>/` | Shot version directory (e.g., v1, v2)                                   |
| `workspace/shots/.../image_*.png`                                                | Generated storyboard images for this shot version                       |
| `workspace/shots/.../tasks/`                                                     | Task-related files directory                                            |
| `workspace/shots/.../tasks/image/`                                               | Image generation task metadata                                          |
| `workspace/shots/.../tasks/image/<task_id>.json`                                 | Image task info (status, progress, file paths)                          |
| `workspace/shots/.../tasks/video/`                                               | Video generation task directory                                         |
| `workspace/shots/.../tasks/video/<task_id>/`                                     | Individual video task directory                                         |
| `workspace/shots/.../tasks/video/<task_id>/metadata.json`                        | Video task metadata                                                     |
| `workspace/shots/.../tasks/video/<task_id>/source.mp4`                           | Generated video file                                                    |
| `workspace/shots/.../tasks/video/<task_id>/first_frame.png`                      | Extracted first frame                                                   |
| `workspace/shots/.../tasks/video/<task_id>/last_frame.png`                       | Extracted last frame                                                    |
| `workspace/assets/`                                                              | Directory containing all asset files                                    |
| `workspace/assets/<asset_id>/`                                                   | Individual asset directory identified by UUID                           |
| `workspace/assets/<asset_id>/character.png`                                      | Character reference sheet (three-view)                                  |
| `workspace/assets/<asset_id>/prop.png`                                           | Prop product image                                                      |
| `workspace/assets/<asset_id>/environment.png`                                    | Environment establishing shot                                           |
| `workspace/assets/<asset_id>/atmosphere.png`                                     | Atmosphere mood board                                                   |
| `workspace/assets/<asset_id>/lighting.png`                                       | Lighting reference diagram                                              |
| `workspace/assets/<asset_id>/products.mp3`                                       | Generated audio file                                                    |
| `[other preserved files]`                                                        | Any additional files from original session                              |
