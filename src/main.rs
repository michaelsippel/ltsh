use {
    laddertypes::*,
    std::io::BufRead
};

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
    let mut dict = TypeDict::new();

    let stdin = std::io::stdin();
    for pipeline in std::io::BufReader::new(stdin).lines() {        
        let mut last_cmd = String::new();
        let mut last_stdout_type_str : Option<String> = None;
        let mut last_stdout_type : Option<TypeTerm> = None;

        let pipeline_str = pipeline.expect("");

        eprintln!("--- BEGIN TYPE-ANALYSIS ---\n");
        for cmd in pipeline_str.split("|") {

            let cmd = cmd.trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ");

            if let Some(intype_str) = get_type_str(&cmd, ">0") {
                let it = dict.parse(&intype_str).expect("parse error");

                if let Some(last) = last_stdout_type {
                    if last.is_syntactic_subtype_of( &it ).is_ok() {
                        eprintln!("* typecheck OK!");
                        eprintln!("    ——————————\n  .... `{}` outputs\n{}    ——————————\n  .... `{}` expects\n{}    ——————————\n",
                                  last_cmd, last_stdout_type_str.unwrap(), cmd, intype_str);
                    } else {
                        eprintln!("* !====> TYPE MISMATCH !! <====!");
                        eprintln!("    ——————————\n  ....`{}` outputs\n{}    ———————————\n  .... `{}` expects\n{}    ——————————\n",
                                  last_cmd, last_stdout_type_str.unwrap(), cmd, intype_str);
                    }
                }
            } else {
                eprintln!("* unknown stdin-type for `{}`\n", cmd);
            }

            if let Some(outtype_str) = get_type_str(&cmd, "<1") {
                let it = dict.parse(&outtype_str).expect("parse error");

                last_stdout_type_str = Some(outtype_str);
                last_stdout_type = Some(it);
            } else {
                eprintln!("* unknown stdout-type for `{}`\n", cmd);
                last_stdout_type_str = None;
                last_stdout_type = None;
            }

            last_cmd = cmd;
        }
    }

    eprintln!("--- END TYPE-ANALYSIS ---\n");
}

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\
