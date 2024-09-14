use anyhow::{Context, Result};
use reqwest::blocking::get;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use zip::ZipArchive;

pub fn check_version(version: &str) -> bool {
    let version_parts: Vec<&str> = version.split('.').collect();
    if version_parts.len() != 3 {
        println!("The version number must be in the format x.y.z");
        return false;
    }

    if version_parts[0] != "3" {
        println!("The major version number must be 3(only support Python 3.x)");
        return false;
    }

    for part in version_parts {
        if !part.chars().all(char::is_numeric) {
            println!("All parts of the version number must be numeric");
            return false;
        }
    }

    true
}

pub fn download_file(url: &str, path: &Path) -> Result<()> {
    let mut resp = get(url).context(format!("Failed to download file from {}", url))?;
    let mut out =
        File::create(path).context(format!("Failed to create file {}", path.display()))?;
    io::copy(&mut resp, &mut out)
        .context(format!("Failed to copy content to {}", path.display()))?;
    Ok(())
}

pub fn unzip_file(zip_path: &Path, dest: &Path) -> Result<()> {
    let file =
        File::open(zip_path).context(format!("Failed to open zip file {}", zip_path.display()))?;
    let mut archive = ZipArchive::new(file)
        .context(format!("Failed to open zip archive {}", zip_path.display()))?;
    archive
        .extract(dest)
        .context(format!("Failed to extract zip file to {}", dest.display()))?;
    Ok(())
}

pub fn modify_pth_file(pth_path: &Path) -> Result<()> {
    let file = File::open(pth_path).context("Failed to open .pth file")?;
    let reader = BufReader::new(file);

    // 创建一个临时文件来写入修改后的内容
    let temp_file_path = "python312._pth.tmp";
    let mut temp_file = File::create(temp_file_path).context("Failed to create temporary file")?;

    // 逐行读取文件内容并进行修改
    for line in reader.lines() {
        let line = line.context("Failed to read line")?;
        if line.contains("import site") {
            // 如果包含，则重写为 "import site"
            writeln!(temp_file, "import site").context("Failed to write to temporary file")?;
        } else {
            // 否则，保持原样
            writeln!(temp_file, "{}", line).context("Failed to write to temporary file")?;
        }
    }

    // 将临时文件重命名为原文件
    fs::rename(temp_file_path, pth_path).context("Failed to rename temporary file")?;

    println!("文件修改完成！");
    Ok(())
}
