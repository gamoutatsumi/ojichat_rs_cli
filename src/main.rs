use clap::*;
use ojichat::ojichat;
use regex::Regex;
use std::io::{stdout, BufWriter, Write};

fn check_num(v: String) -> std::result::Result<(), String> {
    let re = Regex::new(r"\p{N}").unwrap();
    if !re.is_match(&*v) {
        return Err(String::from("絵文字/顔文字の最大連続数が不正です"));
    }
    Ok(())
}

fn main() {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::from_usage("-e [number] '絵文字/顔文字の最大連続数 [default: 4]'")
                .validator(check_num)
                .default_value("4"),
        )
        .arg(
            Arg::from_usage("-p [level] '句読点挿入頻度レベル [min:0, max:3] [default: 0]'")
                .possible_values(&["0", "1", "2", "3"])
                .hide_possible_values(true)
                .default_value("0"),
        )
        .arg(Arg::from_usage("[name] 'おじさんのメール相手'"));

    let matches = app.get_matches();
    let target: Option<String>;

    let emoji_num = Some(value_t!(matches.value_of("e"), usize).unwrap_or_default());

    let punctuation_level = Some(value_t!(matches.value_of("p"), usize).unwrap_or_default());

    if let Some(o) = matches.value_of("name") {
        target = Some(o.to_string());
    } else {
        target = None;
    };

    let message = ojichat::get_message(target, emoji_num, punctuation_level);

    writeln!(out, "{}", message).unwrap();
}
