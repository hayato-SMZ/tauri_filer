// fileSystemのモジュール
// ファイルシステムに関する処理を行う
// ファイルの読み書きやディレクトリの作成など
// このモジュールはsrc-tauri/src/main.rsで使用される

// Pathの文字列が渡されたら、そのディレクトリのファイルとディレクトリの一覧を返す
// この関数はsrc-tauri/src/main.rsのgreet関数から呼び出される
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// DirEntryの情報と、そのDireEntryのプロパティを保持する構造体
#[derive(Serialize, Deserialize)]
pub struct DirEntryInfo {
    pub full_path: String,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    pub file_name: String,
    pub file_size: u64,
    pub file_type: String,     // ファイルの種類
    pub file_modified: String, // ファイルの更新日時
}

pub fn get_dir_entries(path: String) -> Result<Vec<DirEntryInfo>, String> {
    // Pathの文字列をPathBufに変換
    let path = PathBuf::from(path);
    println!("{:?}", path);
    // PathBufがディレクトリかどうかを判定
    if !path.is_dir() {
        // ディレクトリでなければエラーを返す
        return Err(format!("{} is not a directory", path.display()));
    }
    // PathBufがディレクトリなら、そのディレクトリのファイルとディレクトリの一覧を取得
    let entries = fs::read_dir(path)
        .map_err(|e| format!("failed to read dir entries: {}", e))?
        .map(|entry| {
            // ファイルとディレクトリの一覧をDirEntryに変換
            let entry = entry.map_err(|e| format!("failed to read dir entry: {}", e));
            // DirEntryInfoに変換
            let entry = entry.map(|entry| {
                let metadata = entry.metadata().unwrap();
                let file_name = entry.file_name().into_string().unwrap();
                let full_path = entry.path().to_str().unwrap().to_string();
                println!("{:?} is dir?? {:?}", full_path, metadata.is_dir());
                let file_type = if metadata.is_dir() {
                    "directory".to_string()
                } else if metadata.is_file() {
                    "file".to_string()
                } else if metadata.file_type().is_symlink() {
                    "symlink".to_string()
                } else {
                    "unknown".to_string()
                };
                let file_size = metadata.len();
                return DirEntryInfo {
                    full_path,
                    is_dir: metadata.is_dir(),
                    is_file: metadata.is_file(),
                    is_symlink: metadata.file_type().is_symlink(),
                    file_name,
                    file_type,
                    file_size,
                    file_modified: format!(
                        "{}",
                        metadata
                            .modified()
                            .unwrap()
                            .duration_since(std::time::SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                    ),
                };
            });
            entry
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ファイルとディレクトリの一覧を返す
    Ok(entries)
}
