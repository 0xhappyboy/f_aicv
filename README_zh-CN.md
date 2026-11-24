<h1 align="center">
    f_aicv
</h1>
<h4 align="center">
一个基于Rust的*.aicv文件解析引擎，用于全新AI视频无限画布矢量(*.aicv)交换协议.它定义了无限画布图、分镜链接以及双向非线性编辑通信的数据抽象层,并为AI短剧生产管线提供完整的序列化、反序列化和结构化操作能力.
</h4>
<p align="center">
  <a href="https://github.com/0xhappyboy/f_aicv/LICENSE"><img src="https://img.shields.io/badge/Apache-2.0-d1d1f6.svg?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=googledocs&label=license&logoColor=BEC5C9" alt="License"></a>
</p>
<p align="center">
<a href="./README_zh-CN.md">简体中文</a> | <a href="./README.md">English</a>
</p>

## 例子

```rust
// 写 *.aicv 文件（打包会话目录）
FAicv::pack("path/to/session_dir", "output.aicv")?;
// 展开 *.aicv 文件（解包到目录）
FAicv::unpack("output.aicv", "restored_session")?;
// 读元数据（不解包，直接读取）
let metadata = FAicv::read_metadata("output.aicv")?;
// 获取归档摘要
let summary = FAicv::summary("output.aicv")?;
// 验证归档文件
let is_valid = FAicv::validate("output.aicv")?;
```

## 归档结构（.aicv 解包后）

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

### 目录说明

| 路径                                                                             | 说明                                         |
| -------------------------------------------------------------------------------- | -------------------------------------------- |
| `session_dir/`                                                                   | AI Canvas 会话根目录                         |
| `workspace/`                                                                     | 核心工作区，包含所有会话数据                 |
| `workspace/metadata.json`                                                        | 主元数据文件，包含剧集、场次、分镜、资产结构 |
| `workspace/shots/`                                                               | 所有分镜媒体文件目录                         |
| `workspace/shots/<shot_id>/`                                                     | 单个分镜目录，通过 UUID 标识                 |
| `workspace/shots/<shot_id>/e<episode_version>/`                                  | 剧集版本目录（如 e1、e2、e3）                |
| `workspace/shots/<shot_id>/e<episode_version>/s<scene_version>/`                 | 场次版本目录（如 s1、s2）                    |
| `workspace/shots/<shot_id>/e<episode_version>/s<scene_version>/v<shot_version>/` | 分镜版本目录（如 v1、v2）                    |
| `workspace/shots/.../image_*.png`                                                | 该分镜版本生成的分镜图                       |
| `workspace/shots/.../tasks/`                                                     | 任务相关文件目录                             |
| `workspace/shots/.../tasks/image/`                                               | 图像生成任务元数据                           |
| `workspace/shots/.../tasks/image/<task_id>.json`                                 | 图像任务信息（状态、进度、文件路径）         |
| `workspace/shots/.../tasks/video/`                                               | 视频生成任务目录                             |
| `workspace/shots/.../tasks/video/<task_id>/`                                     | 单个视频任务目录                             |
| `workspace/shots/.../tasks/video/<task_id>/metadata.json`                        | 视频任务元数据                               |
| `workspace/shots/.../tasks/video/<task_id>/source.mp4`                           | 生成的视频文件                               |
| `workspace/shots/.../tasks/video/<task_id>/first_frame.png`                      | 提取的首帧                                   |
| `workspace/shots/.../tasks/video/<task_id>/last_frame.png`                       | 提取的尾帧                                   |
| `workspace/assets/`                                                              | 所有资产文件目录                             |
| `workspace/assets/<asset_id>/`                                                   | 单个资产目录，通过 UUID 标识                 |
| `workspace/assets/<asset_id>/character.png`                                      | 角色三视图参考表                             |
| `workspace/assets/<asset_id>/prop.png`                                           | 道具产品图                                   |
| `workspace/assets/<asset_id>/environment.png`                                    | 环境 establishing shot                       |
| `workspace/assets/<asset_id>/atmosphere.png`                                     | 氛围 mood board                              |
| `workspace/assets/<asset_id>/lighting.png`                                       | 灯光参考图                                   |
| `workspace/assets/<asset_id>/products.mp3`                                       | 生成的音频文件                               |
| `[other preserved files]`                                                        | 原始会话中的其他保留文件                     |
