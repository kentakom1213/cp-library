use clap::Parser;
use log;
use std::{env, fs, path::Path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

/// ライブラリのパスを取得する
fn get_library_path() -> String {
    let mut cur = env::current_dir().unwrap();
    cur.pop();
    cur.push("cp-library-rs");
    cur.to_str().unwrap().to_string()
}

fn main() {
    // ロガーの初期化
    env_logger::init();

    // 引数の取得
    let args = Args::parse();

    // ライブラリのパスを取得
    let library_path = get_library_path();
    log::info!("library_path: {:?}", library_path);

    let input_path = Path::new(&args.input);
    log::info!("input_path: {:?}", input_path);

    // ファイルの中身を取得
    let Ok(input_file) = fs::read_to_string(&input_path) else {
        panic!("No such file: {:?}.", &input_path);
    };

    
}
