use clap::{Parser, Subcommand};
use reqwest::blocking::get;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use zip::ZipArchive;

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
        #[arg(short, long)]
        version: String,
    },
    Install {
        #[arg(short, long)]
        proxy: Option<String>,
        packages: Vec<String>,
    },
}

fn download_file(url: &str, path: &Path) -> io::Result<()> {
    let mut resp = get(url).expect("Failed to download file");
    let mut out = File::create(path)?;
    io::copy(&mut resp, &mut out)?;
    Ok(())
}

fn unzip_file(zip_path: &Path, dest: &Path) -> io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(dest)?;
    Ok(())
}

fn modify_pth_file(pth_path: &Path) -> io::Result<()> {
    let file = File::open(pth_path)?;
    let reader = BufReader::new(file);

    // 创建一个临时文件来写入修改后的内容
    let temp_file_path = "python312._pth.tmp";
    let mut temp_file = File::create(temp_file_path)?;

    // 逐行读取文件内容并进行修改
    for line in reader.lines() {
        let line = line?;
        if line.contains("import site") {
            // 如果包含，则重写为 "import site"
            writeln!(temp_file, "import site")?;
        } else {
            // 否则，保持原样
            writeln!(temp_file, "{}", line)?;
        }
    }

    // 将临时文件重命名为原文件
    fs::rename(temp_file_path, pth_path)?;

    println!("文件修改完成！");
    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { version } => {
            let python_url = format!(
                "https://www.python.org/ftp/python/{}/python-{}-embed-amd64.zip",
                version, version
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
            let python_path = Path::new("python.exe").canonicalize()?;

            let output = std::process::Command::new(python_path)
                .arg("get-pip.py")
                .output()
                .expect("Failed to execute");

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
            let python_path = Path::new("python.exe").canonicalize()?;
            println!("Python path: {:?}", python_path);
            let mut cmd = std::process::Command::new(python_path);
            cmd.arg("-m").arg("pip").arg("install");

            if let Some(proxy_url) = proxy {
                cmd.arg(format!("--proxy={}", proxy_url));
            }

            for package in packages {
                cmd.arg(package);
            }

            let output = cmd.output().expect("Failed to run pip install");

            io::stdout().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;
        }
    }

    Ok(())
}
