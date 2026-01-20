use clap::Parser;
use expander::{cargo_lock::resolve_git_library_path, expander::ModuleExpander};
use std::{path::PathBuf, process::exit};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path or alias of input file
    input: String,
    /// library path (exclusive with --crate-name)
    #[arg(short, long)]
    library: Option<PathBuf>,
    /// crate name to resolve from Cargo.lock (exclusive with --library)
    #[arg(long)]
    crate_name: Option<String>,
    #[arg(short, long)]
    inplace: bool,
    /// restore file from backup
    #[arg(short, long)]
    restore: bool,
}

fn main() {
    // ロガーの初期化
    env_logger::init();

    // 引数の取得
    let args = Args::parse();

    // 入力ファイルのパスを取得
    let input_path = PathBuf::from(&args.input);
    log::info!("input_path: {:?}", input_path);

    let library_path = match (&args.library, &args.crate_name) {
        (Some(_), Some(_)) => {
            log::error!("--library and --crate-name are mutually exclusive");
            exit(1);
        }
        (None, None) => {
            log::error!("--library or --crate-name must be provided");
            exit(1);
        }
        (Some(library), None) => {
            log::info!("library_path: {:?}", library);
            library.clone()
        }
        (None, Some(crate_name)) => match resolve_git_library_path(&input_path, crate_name) {
            Ok(path) => {
                log::info!("library_path: {:?}", path);
                path
            }
            Err(err) => {
                log::error!("failed to resolve git library path");
                log::error!("{err}");
                exit(1);
            }
        },
    };

    // expanderの初期化
    let mut expander = match ModuleExpander::new(input_path.clone(), library_path) {
        Ok(expander) => expander,
        Err(err) => {
            log::error!("expander initialize failed");
            log::error!("{err}");
            exit(1);
        }
    };

    log::info!("library_name: {}", expander.library_name);
    log::info!("import_name: {}", expander.import_name);

    if args.restore {
        match expander.restore() {
            Ok(_) => log::info!("restore complete"),
            Err(err) => {
                log::error!("restore failed");
                log::error!("{err}");
                exit(1);
            }
        }
    } else if args.inplace {
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
    } else {
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
}
