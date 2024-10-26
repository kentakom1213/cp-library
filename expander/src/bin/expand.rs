use anyhow::bail;
use clap::Parser;
use expander::expander::ModuleExpander;
use std::{env, path::PathBuf, process::exit};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// library path
    library: PathBuf,
    /// path or alias of input file
    input: String,
    /// export file to stream
    #[arg(short, long)]
    export: bool,
    /// restore file from backup
    #[arg(short, long)]
    restore: bool,
}

fn main() {
    // ロガーの初期化
    env_logger::init();

    // 引数の取得
    let args = Args::parse();

    // ライブラリのパスを取得
    log::info!("library_path: {:?}", args.library);

    // 入力ファイルのパスを取得
    let mut input_path = PathBuf::from(&args.input);
    log::info!("input_path: {:?}", input_path);

    // 存在しない場合はエイリアスとして処理
    if !input_path.exists() {
        // 最後の'-'をスペースに置換
        let dir_file = args
            .input
            .chars()
            .rev()
            .collect::<String>()
            .replacen('-', " ", 1)
            .chars()
            .rev()
            .collect::<String>();

        // ディレクトリ名部分とファイル名部分に分割
        let &[contest, bin, ..] = &dir_file.split(' ').collect::<Vec<&str>>()[..] else {
            log::error!("invalid input: {:?}", args.input);
            exit(1);
        };

        let mut buf = PathBuf::from(env::var("KYOPRO_DIR").unwrap());
        buf.push(&format!("contests/{contest}/src/bin/{bin}.rs"));

        input_path = buf;

        log::info!("bin_alias: {:?}", input_path);
    }

    // expanderの初期化
    let mut expander = match ModuleExpander::new(input_path.clone(), args.library) {
        Ok(expander) => expander,
        Err(err) => {
            log::error!("expander initialize failed");
            log::error!("{err}");
            exit(1);
        }
    };

    log::info!("library_name: {}", expander.library_name);
    log::info!("import_name: {}", expander.import_name);

    if args.export {
        match expander.expand() {
            Ok(contents) => {
                // ストリームに出力
                println!("{contents}");
                exit(0);
            }
            Err(err) => {
                log::error!("expand failed");
                log::error!("{err}");
                exit(1);
            }
        }
    }

    if !args.restore {
        match expander.expand_inplace() {
            Ok(_) => {
                log::info!("expand complete");
            }
            Err(err) => {
                log::error!("expand failed");
                log::error!("{err}");
                exit(1);
            }
        }
    }

    if args.restore {
        match expander.restore() {
            Ok(_) => log::info!("restore complete"),
            Err(err) => {
                log::error!("restore failed");
                log::error!("{err}");
                exit(1);
            }
        }
    }
}
