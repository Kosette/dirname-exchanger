use std::env;
use std::fs;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Please provide two directory paths as arguments");
        return;
    }

    let dir1 = &args[1];
    let dir2 = &args[2];

    // 交换目录的名字
    if let Err(err) = swap_directories(dir1, dir2) {
        eprintln!("Error: {}", err);
    }
}

fn swap_directories(dir1: &str, dir2: &str) -> Result<(), std::io::Error> {
    // 检查目录是否存在
    if !is_directory_valid(dir1) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Invalid directory path: '{}'", dir1),
        ));
    }

    if !is_directory_valid(dir2) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Invalid directory path: '{}'", dir2),
        ));
    }

    // 生成交换目录的临时名字
    let temp_dir = format!("{}_temp", dir1);

    // 重命名目录1为临时名字
    fs::rename(dir1, &temp_dir)?;

    // 重命名目录2为目录1的名字
    fs::rename(dir2, dir1)?;

    // 重命名临时目录为目录2的名字
    fs::rename(&temp_dir, dir2)?;

    Ok(())
}

fn is_directory_valid(path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        metadata.is_dir()
    } else {
        false
    }
}
