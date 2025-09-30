use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::fs::{self, File};
/// 一些有用的 api

/// 检查文件是否存在，不存在就创建
pub async fn create_file_if_not_exists(file: &PathBuf) -> Result<()> {
    // 检查文件是否已存在
    if file.exists() {
        return Ok(());
    }

    // 获取父目录路径
    if let Some(parent) = file.parent() {
        // 递归创建所有父目录（如果不存在）
        tokio::fs::create_dir_all(parent).await?;
    }

    // 创建文件
    File::create(file).await?;
    println!("Created file: {:?}", file);
    Ok(())
}

/// 异步保存文件内容，自动创建所需目录
///
/// # 参数
/// - `path`: 文件路径
/// - `contents`: 要写入的文件内容（可以是字符串或字节数据）
///
/// # 示例
/// ```
/// #[tokio::main]
/// async fn main() -> io::Result<()> {
///     let path = Path::new("data/output.txt");
///     let content = "Hello, async file saving!";
///     save_file(path, content).await?;
///     Ok(())
/// }
/// ```
pub async fn save_file<P, C>(path: P, contents: C) -> Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    let path = path.as_ref();

    // 确保父目录存在
    if let Some(parent) = path.parent() {
        // 异步创建所有缺失的父目录
        fs::create_dir_all(parent).await?;
    }

    // 异步写入文件内容
    fs::write(path, contents).await?;

    println!("File saved successfully: {}", path.display());
    Ok(())
}
