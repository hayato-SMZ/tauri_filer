// fileSystemのモジュール
// ファイルシステムに関する処理を行う
// ファイルの読み書きやディレクトリの作成など
// このモジュールはsrc-tauri/src/main.rsで使用される

// Pathの文字列が渡されたら、そのディレクトリのファイルとディレクトリの一覧を返す
// この関数はsrc-tauri/src/main.rsのgreet関数から呼び出される
use std::path::PathBuf;
use std::fs::DirEntry;
use std::fs;

pub fn get_dir_entries(path: String) -> Result<Vec<DirEntry>, String> {
    // Pathの文字列をPathBufに変換
    let path = PathBuf::from(path);
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
            // DirEntryを返す
            entry
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ファイルとディレクトリの一覧を返す
    Ok(entries)
}