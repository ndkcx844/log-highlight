use std::fs::File;
use std::io::{BufRead, BufReader, stdout, Write, BufWriter};
use std::error::Error;
use std::io::ErrorKind;
use crate::rule::Rule;

// NOTE: regexよりも約1.4倍程早かったのでpcre2を使う
use pcre2::bytes::Regex;
use pcre2::bytes::RegexBuilder;

const RESET: &str = "\x1b[0m";

pub struct Replacer {
    regex: Regex,
    color: String,
}

pub struct Highlighter {
    replacers: Vec<Replacer>,
}

impl Highlighter {
    pub fn new(rules: Vec<Rule>) -> Result<Self, Box<dyn Error>> {
        let mut replacers: Vec<Replacer> = Vec::new();
        for rule in rules {
            // NOTE: caseless(true)すると約10%遅くなるのでパターン側で
            //       インライン修飾子を指定する
            let regex = RegexBuilder::new().jit(true).build(&rule.pattern)?;
            let replacer = Replacer{ regex: regex, color: rule.color };
            replacers.push(replacer);
        }
        Ok(Highlighter{replacers})
    }

    pub fn highlight(&self, files: Vec<String>) -> Result<(), Box<dyn Error>> {
        for file in files {
            self.highlight_file(file)?;
        }
        Ok(())
    }

    fn highlight_file(&self, filepath: String) -> Result<(), Box<dyn Error>> {
        let reader = File::open(filepath).map(|file| BufReader::new(file))?;

        // NOTE: println!()はロックが無駄なのでwriteln!()を使う
        let out = stdout();
        let mut out = BufWriter::new(out.lock());

        // XXX: バッファを使いまわしてread_line()するほうが早いらしいが、
        //      所有権を保持したままバイト列に変換する方法がわからなかった
        //      clone()でコピーするならイテレータと変わらないはず
        for line in reader.lines() {
            let mut line_pretty: Vec<u8> = line?.into_bytes();
            let mut line_buffer: Vec<u8> = Vec::with_capacity(line_pretty.len());
            for replacer in &self.replacers {
                let mut iter = replacer.regex.captures_iter(&line_pretty).peekable();
                if ! iter.peek().is_some() {
                    continue;
                }
                let mut index = 0;
                line_buffer.clear();
                for walk in iter {
                    let caps = walk?;
                    // (${HEAD})(${PATTERN})(${TAIL})なのでインデックスは2
                    let matched = caps.get(2).unwrap();
                    // パターンより前の文字列を結合
                    line_buffer.extend_from_slice(&line_pretty[index..matched.start()]);
                    let after = format!("{0}{1}{2}", replacer.color, std::str::from_utf8(&caps[2])?, RESET).into_bytes();
                    // 置換後の文字列を結合
                    line_buffer.extend_from_slice(&after);
                    index = matched.end();
                }
                // パターンより後の文字列を結合
                line_buffer.extend_from_slice(&line_pretty[index..]);

                line_pretty.clear();
                line_pretty.extend(&line_buffer);
            }
            let ret = writeln!(out, "{}", std::str::from_utf8(&line_pretty)?);
            match ret {
                Ok(_) => (),
                Err(ref error) if error.kind() == ErrorKind::BrokenPipe => {
                    // If broken pipe. Exit gracefully.
                    break;
                },
                Err(error) => Err(error)?,
            };
        }

        Ok(())
    }
}
