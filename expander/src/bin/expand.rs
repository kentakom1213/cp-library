use clap::Parser;
use expander::utils::{get_library_path, ModuleExpander};
use std::{io, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    input: String,
    /// Restore file
    #[arg(short, long)]
    restore: bool,
}

fn main() -> Result<(), Box<io::Error>> {
    // ロガーの初期化
    env_logger::init();

    // 引数の取得
    let args = Args::parse();

    // ライブラリのパスを取得
    let library_path = get_library_path();
    log::info!("library_path: {:?}", library_path);

    assert!(library_path.exists());

    // 入力ファイルのパスを取得
    let input_path = PathBuf::from(&args.input);
    log::info!("input_path: {:?}", input_path);

    assert!(input_path.exists());

    // expanderの初期化
    let mut expander = ModuleExpander::new(input_path, library_path);

    if !args.restore {
        expander.expand()?;
        log::info!("expand");
    } else {
        expander.restore()?;
        log::info!("restore");
    }

    Ok(())
}
