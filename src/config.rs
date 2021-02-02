use std::env;
use std::error::Error;
use crate::rule::{Rules, Rule};

pub struct Config {
    pub files: Vec<String>,
    pub rules: Vec<Rule>,
    pub show_help: bool,
    pub show_rules: bool,
}

impl Config {
    pub fn new(args: env::Args) -> Result<Self, Box<dyn Error>> {
        let mut files: Vec<String> = Vec::new();

        // TODO: オプション解析のライブラリ(clap)化
        // https://qiita.com/emonuh/items/41f7bba5283c732b0209
        // https://github.com/DiD92/rust-minigrep/blob/master/src/lib.rs
        // https://ubnt-intrepid.hatenablog.com/entry/rust_commandline_parsers
        let args: Vec<_> = args.collect();
        let mut conf: String = "".to_string();
        let mut show_help: bool = false;
        let mut show_rules: bool = false;
        // NOTE: 第一引数は実行パスなので読み飛ばす
        let mut index = 1;
        while index < args.len() {
            let arg = &args[index];
            match &arg[..] {
                "-h" | "--help" | "--usage" => {
                    show_help = true;
                },
                "--show-rules" | "--dump-rules" => {
                    show_rules = true;
                },
                "-c" | "--config" => {
                    index += 1;
                    if index >= args.len() {
                        Err("missing FILE parameter for --config option")?
                    };
                    conf = args[index].to_string();
                },
                "-" => files.push("/dev/stdin".to_string()),
                _ => {
                    if arg.starts_with("-") {
                        Err(format!("unrecognized option: {}", arg))?
                    }
                    files.push(arg.to_string());
                },
            }
            index += 1;
        }

        if ! show_help && ! show_rules {
            // NOTE: 標準入力から読み込み(ファイル名指定なしの場合)
            if files.len() == 0 {
                files.push("/dev/stdin".to_string());
            }
        }

        // 置換ルールの作成
        let rules = Rules::new(&conf)?;

        Ok(Config{files, rules: rules.rules, show_help, show_rules})
    }
}
