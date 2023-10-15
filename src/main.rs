use {
    laddertypes::*,
    std::io::BufRead,
    tiny_ansi::TinyAnsi
};

mod ast;
mod env;
mod parse;
//mod expand;

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\

pub fn get_type_str(cmd: &str, item: &str) -> Option<String> {
    let gt = String::from(env!("CARGO_MANIFEST_DIR")) + "/gettype.sh";
    let stdout_typeladder_str = std::process::Command::new(gt)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg(cmd)
        .arg(item)
        .output()
        .ok()?
        .stdout;

    let s = String::from_utf8(stdout_typeladder_str).ok()?;
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\

fn main() {
    let mut success = true;
    let mut dict = TypeDict::new();

    let stdin = std::io::stdin();
    for line in std::io::BufReader::new(stdin).lines() {
        if let Ok(line) = line {
            let mut lex = parse::WordLexer::from( line.chars() );
            for word in lex {
                eprintln!("word-segment: {:?}", word);
            }
        }
    }

    return;

    let stdin = std::io::stdin();
    for pipeline in std::io::BufReader::new(stdin).lines() {
        let mut last_cmd = String::new();
        let mut last_stdout_type : Option<TypeTerm> = None;

        let pipeline_str = pipeline.expect("");

        eprintln!("{}", "--- BEGIN TYPE-ANALYSIS ---\n".blue());
        for cmd in pipeline_str.split("|") {

            let cmd = cmd.trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ");

            if let Some(intype_str) = get_type_str(&cmd, ">0") {
                let it = dict.parse(&intype_str).expect("parse error");

                if let Some(last) = last_stdout_type {
                    match last.is_syntactic_subtype_of( &it ) {
                        Ok(first_match) => {
                            eprintln!("{} typecheck {}", "*".bright_blue().bold(), "ok".bold().green());

                            let rl = last.get_lnf_vec().iter().map(|t| dict.unparse(t)).collect::<Vec<_>>();
                            let rr = it.get_lnf_vec().iter().map(|t| dict.unparse(t)).collect::<Vec<_>>();

                            let c1_width = usize::max(rl.iter().map(|s| s.chars().count()).max().unwrap_or(0), last_cmd.chars().count());
                            for _ in last_cmd.chars().count() .. c1_width { eprint!(" "); }
                            eprintln!("{}{}{}", last_cmd.on_black().bright_blue(), " | ".on_black().yellow().bold(), cmd.on_black().bright_blue());

                            for i in 0 .. rl.len() {
                                if i < first_match {
                                    eprint!("{}", rl[i].bright_black());

                                    for _ in rl[i].chars().count() .. c1_width { eprint!(" "); }
                                    eprintln!(" {}", "|".bright_black());
                                } else {
                                    eprint!("{}", rl[i].green());

                                    if i-first_match < rr.len() {
                                        for _ in rl[i].chars().count() .. c1_width { eprint!(" "); }
                                        eprintln!(" | {}", rr[i-first_match].green());
                                    } else {
                                        eprintln!("");
                                    }
                                }
                            }
                            eprintln!("");
                        }
                        Err((first_match, first_mismatch)) => {
                            success = false;

                            eprintln!("{} typecheck {}", "*".bright_blue().bold(), "error".bold().red());

                            let rl = last.get_lnf_vec().iter().map(|t| dict.unparse(t)).collect::<Vec<_>>();
                            let rr = it.get_lnf_vec().iter().map(|t| dict.unparse(t)).collect::<Vec<_>>();

                            let c1_width = usize::max(rl.iter().map(|s| s.chars().count()).max().unwrap_or(0), last_cmd.chars().count());
                            for _ in last_cmd.chars().count() .. c1_width { eprint!(" "); }
                            eprintln!("{}{}{}", last_cmd.on_black().bright_blue(), " | ".on_black().yellow().bold(), cmd.on_black().bright_blue());

                            for i in 0 .. rl.len() {
                                if i < first_match {
                                    eprint!("{}", &rl[i].bright_black());

                                    for _ in rl[i].chars().count() .. c1_width { eprint!(" "); }
                                    eprintln!(" {}", "|".bright_black());
                                } else {
                                    if i < first_mismatch {
                                        eprint!("{}", rl[i].green());

                                        if i - first_match < rr.len() {
                                            for _ in rl[i].chars().count() .. c1_width { eprint!(" "); }
                                            eprintln!(" | {}", rr[i-first_match].green());
                                        } else {
                                            eprintln!("");
                                        }
                                    } else if i > first_mismatch {
                                        eprint!("{}", rl[i].bright_black());

                                        if i - first_match < rr.len() {
                                            for _ in rl[i].chars().count() .. c1_width { eprint!(" "); }
                                            eprintln!(" | {}", rr[i-first_match].bright_black());
                                        } else {
                                            eprintln!("");
                                        }
                                    } else {
                                        eprint!("{}", rl[i].red());

                                        if i - first_match < rr.len() {
                                            for _ in rl[i].chars().count() .. c1_width { eprint!(" "); }
                                            eprintln!(" | {}", rr[i-first_match].red());
                                        } else {
                                            eprintln!("");
                                        }
                                    }
                                }
                            }
                            eprintln!("");
                        }
                    }
                }
            } else {
                eprintln!("{} {} stdin-type of `{}`\n", "*".bold().bright_blue(), "unknown".yellow(), cmd.on_black().bright_blue());
            }

            if let Some(outtype_str) = get_type_str(&cmd, "<1") {
                let it = dict.parse(&outtype_str).expect("parse error");
                last_stdout_type = Some(it);
            } else {
                eprintln!("{} {} stdout-type of `{}`\n", "*".bold().bright_blue(), "unknown".yellow(), cmd.on_black().bright_blue());
                last_stdout_type = None;
            }

            last_cmd = cmd;
        }
        eprintln!("{}", "--- END TYPE-ANALYSIS ---".blue());
    }

    std::process::exit(
        if success {
            0
        } else {
            1
        }
    );
}

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\
