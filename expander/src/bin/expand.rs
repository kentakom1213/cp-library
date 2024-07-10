use clap::Parser;
use expander::utils::{get_library_path, ModuleExpander};
use std::{env, fs, io, path::PathBuf, process::exit};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path or alias of input file
    input: String,
    /// export file to stream
    #[arg(short, long)]
    export: bool,
    /// restore file from backup
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

    // 入力ファイルのパスを取得
    let mut input_path = PathBuf::from(&args.input);
    log::info!("input_path: {:?}", input_path);

    // 存在しない場合はエイリアスとして処理
    if !input_path.exists() {
        let &[contest, bin, ..] = &args.input.split('-').collect::<Vec<&str>>()[..] else {
            panic!("No such contest or binary.");
        };

        let mut buf = PathBuf::from(env::var("KYOPRO_DIR").unwrap());
        buf.push(&format!("contests/{contest}/src/bin/{bin}.rs"));

        input_path = buf;

        log::info!("bin_alias: {:?}", input_path);
    }

    // expanderの初期化
    let mut expander = ModuleExpander::new(input_path.clone(), library_path);

    if !args.restore {
        match expander.expand() {
            Ok(_) => {
                log::info!("expand complete");

                // ストリームに出力
                if args.export {
                    let output = fs::read_to_string(&input_path)?;
                    println!("{output}");
                }
            }
            Err(err) => {
                log::error!("expand failed");
                log::error!("{err}");
                exit(1);
            }
        }
    } else {
        match expander.restore() {
            Ok(_) => log::info!("restore complete"),
            Err(err) => {
                log::error!("restore failed");
                log::error!("{err}");
                exit(1);
            }
        }
    }

    Ok(())
}
