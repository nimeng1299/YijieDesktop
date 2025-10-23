use anyhow::{bail, Result};
use chrono::{Datelike, Local, Timelike};
use std::path::{Path, PathBuf};
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

/// 获取当前可执行文件的目录
pub fn get_current_exe_dir() -> Result<PathBuf> {
    match std::env::current_exe() {
        Ok(exe_path) => {
            if let Some(dir) = exe_path.parent() {
                Ok(dir.to_path_buf())
            } else {
                bail!("获取当前目录失败, exe_path为空")
            }
        }
        Err(e) => bail!("获取当前目录失败, {e}"),
    }
}

/// 获取格式化后的时间
pub fn get_foramt_time() -> String {
    // 获取当前本地时间
    let now = Local::now();

    // 分别提取时间分量
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();
    let second = now.second();

    format!(
        "{}-{:02}-{:02} {:02}-{:02}-{:02}",
        year, month, day, hour, minute, second
    )
}

// 递归遍历目录
pub fn visit_dirs<F>(dir: &Path, cb: &F) -> std::io::Result<()>
where
    F: Fn(&Path),
{
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?; // 递归调用
            } else {
                cb(&path); // ✅ 调用传入的闭包
            }
        }
    }
    Ok(())
}
