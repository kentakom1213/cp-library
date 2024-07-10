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
        match expander.expand() {
            Ok(_) => log::info!("expand complete"),
            Err(err) => {
                log::error!("expand failed");
                log::error!("{err}");
            }
        }
    } else {
        match expander.restore() {
            Ok(_) => log::info!("restore complete"),
            Err(err) => {
                log::error!("restore failed");
                log::error!("{err}");
            }
        }
    }

    Ok(())
}
