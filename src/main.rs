use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use repy::{check_version, download_file, modify_pth_file, unzip_file};
use std::path::Path;

#[derive(Parser)]
#[command(name = "repy")]
#[command(about = "A CLI tool to manage embedded Python environments", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long, default_value = "3.12.6")]
        version: String,
        #[arg(short, long, value_parser = ["win32", "amd64", "arm64"], default_value = "amd64")]
        arch: String,
    },
    Install {
        #[arg(short, long)]
        proxy: Option<String>,
        packages: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { version, arch } => {
            // 进行版本号的校验
            if !check_version(version) {
                anyhow::bail!("Invalid version number");
            }

            let python_url = format!(
                "https://www.python.org/ftp/python/{}/python-{}-embed-{}.zip",
                version, version, arch
            );
            let get_pip_url = "https://bootstrap.pypa.io/get-pip.py";

            let python_zip_path = Path::new("python_embed.zip");
            let get_pip_path = Path::new("get-pip.py");

            println!("Downloading Python {}...", version);
            download_file(&python_url, python_zip_path)?;

            println!("Downloading get-pip.py...");
            download_file(get_pip_url, get_pip_path)?;

            println!("Unzipping Python...");
            unzip_file(python_zip_path, Path::new("."))?;

            // 提取版本号的x.y.z -> xy
            let version_parts: Vec<&str> = version.split('.').collect();
            let pth_name = format!("python{}{}._pth", version_parts[0], version_parts[1]);

            println!("Modifying {}...", pth_name);
            modify_pth_file(Path::new(&pth_name))?;

            println!("Running get-pip.py...");
            // 获取python.exe的绝对路径
            let python_path = Path::new("python.exe")
                .canonicalize()
                .context("Failed to get canonical path for python.exe")?;

            let output = std::process::Command::new(python_path)
                .arg("get-pip.py")
                .output()
                .context("Failed to execute get-pip.py")?;

            if output.status.success() {
                // 将命令的标准输出转换为字符串并打印
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
            } else {
                // 如果命令失败，打印错误信息
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Command failed: {}", stderr);
            }

            println!("Python environment setup complete.");
        }
        Commands::Install { proxy, packages } => {
            let python_path = Path::new("python.exe")
                .canonicalize()
                .context("Failed to get canonical path for python.exe")?;

            let mut cmd = std::process::Command::new(python_path);
            cmd.arg("-m").arg("pip").arg("install");

            if let Some(proxy_url) = proxy {
                cmd.arg(format!("--proxy={}", proxy_url));
            }

            for package in packages {
                cmd.arg(package);
            }

            let output = cmd.output().context("Failed to run pip install")?;

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Command failed: {}", stderr);
            }
        }
    }

    Ok(())
}
