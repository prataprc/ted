use autocfg;
use cc;
use which;

use std::{env, ffi, path, process};

#[macro_export]
macro_rules! check_exit {
    ($res:expr, $n:expr) => {{
        match $res {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err);
                process::exit($n);
            }
        }
    }};
}

#[macro_export]
macro_rules! err_at {
    (msg:$m:expr) => {
        Err(format!("{}:{} {}", file!(), line!(), $m))
    };
    ($e:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => Err(format!("{}:{} err:{:?}", file!(), line!(), err)),
        }
    };
    ($e:expr, $m:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                //
                Err(format!("{}:{} {} err:{:?}", file!(), line!(), $m, err))
            }
        }
    };
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    check_exit!(auto_cfg_npm(), 1);
    check_exit!(install_npm_pkgs(), 2);

    let parsers = vec![
        Parser {
            name: "txt_plain".to_string(),
            dir: "src/tss/txt_plain".into(),
            grammar: "src/tss/txt_plain/grammar.js".into(),
            sources: vec!["src/tss/txt_plain/src/parser.c".into()],
        },
        Parser {
            name: "tss".to_string(),
            dir: "src/tss/tss".into(),
            grammar: "src/tss/tss/grammar.js".into(),
            sources: vec!["src/tss/tss/src/parser.c".into()],
        },
        Parser {
            name: "toml".to_string(),
            dir: "src/tss/toml".into(),
            grammar: "src/tss/toml/grammar.js".into(),
            sources: vec![
                "src/tss/toml/src/parser.c".into(),
                "src/tss/toml/src/scanner.c".into(),
            ],
        },
        Parser {
            name: "code_cmd".to_string(),
            dir: "src/tss/code_cmd".into(),
            grammar: "src/tss/code_cmd/grammar.js".into(),
            sources: vec!["src/tss/code_cmd/src/parser.c".into()],
        },
    ];
    for parser in parsers.into_iter() {
        check_exit!(build_parser(parser), 3);
    }
}

fn auto_cfg_npm() -> Result<(), String> {
    match which::which("npm") {
        Ok(_) => {
            autocfg::emit("npm");
            Ok(())
        }
        Err(err) => Err(format!(
            "Error: install `npm` to compile tree-sitter grammars: {}",
            err
        )),
    }
}

fn install_npm_pkgs() -> Result<(), String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let _sd = SwitchDir::to_manifest_dir();

    let commands = vec![
        ("npm", vec!["install", "--save", "nan"], "npm --save nan"),
        (
            "npm",
            vec!["install", "--save-dev", "tree-sitter-cli"],
            "npm --save-dev tree-sitter-cli",
        ),
        (
            "npm",
            vec!["install", "--save", "regexp-util"],
            "npm --save regexp-util",
        ),
    ];
    for c in commands.into_iter() {
        match err_at!(process::Command::new(c.0).args(&c.1).status())? {
            n if n.success() => Ok(()),
            n => Err(format!("Error: {:?} exited with {}", c.2, n)),
        }?;
    }

    let paths = {
        let node_bin = path::Path::new(&manifest_dir)
            .join("node_modules")
            .join(".bin");
        match env::var_os("PATH") {
            Some(path) => {
                let mut paths = vec![&node_bin];
                let env_paths = env::split_paths(&path).collect::<Vec<_>>();
                paths.extend(&env_paths);
                err_at!(env::join_paths(paths))?
            }
            None => {
                let paths = vec![&node_bin];
                err_at!(env::join_paths(paths))?
            }
        }
    };
    env::set_var("PATH", paths);

    Ok(())
}

struct Parser {
    name: String,
    dir: ffi::OsString,
    grammar: ffi::OsString,
    sources: Vec<ffi::OsString>,
}

fn build_parser(parser: Parser) -> Result<(), String> {
    // generate
    {
        let _sd = SwitchDir::to_lang_dir(&parser.dir);
        let cmd = "tree-sitter generate";
        match err_at!(process::Command::new("tree-sitter")
            .args(&["generate"])
            .status())?
        {
            n if n.success() => Ok(()),
            n => Err(format!("Error: {:?} exited with {}", cmd, n)),
        }?;
    }
    // mark for rerun
    println!(
        "cargo:rerun-if-changed={}",
        parser.grammar.to_str().unwrap()
    );
    // build library
    {
        let _sd = SwitchDir::to_manifest_dir();
        let mut b = cc::Build::new();
        b.warnings(false); // TODO: is this good practise ?
        b.include(err_at!(env::current_dir())?.join(parser.dir).join("src"));
        b.include(
            ["/", "usr", "include", "nodejs", "src"]
                .iter()
                .collect::<path::PathBuf>(),
        );
        b.include(
            ["/", "usr", "include", "nodejs", "deps", "v8", "include"]
                .iter()
                .collect::<path::PathBuf>(),
        );
        b.include(
            err_at!(env::current_dir())?
                .join("node_modules")
                .join("nan"),
        );
        for file in parser.sources.iter() {
            b.file(file);
        }
        b.compile(&parser.name);
    }
    // emit configuration
    autocfg::emit(&parser.name);

    Ok(())
}

struct SwitchDir {
    cwd: ffi::OsString,
    _to_dir: ffi::OsString,
}

impl Drop for SwitchDir {
    fn drop(&mut self) {
        env::set_current_dir(self.cwd.clone()).unwrap();
    }
}

impl SwitchDir {
    fn to_manifest_dir() -> Result<SwitchDir, String> {
        let cwd = err_at!(env::current_dir())?;
        let to_dir = env!("CARGO_MANIFEST_DIR");
        err_at!(env::set_current_dir(to_dir))?;
        Ok(SwitchDir {
            cwd: cwd.into_os_string(),
            _to_dir: to_dir.into(),
        })
    }

    fn to_lang_dir(to_dir: &ffi::OsStr) -> Result<SwitchDir, String> {
        let cwd = err_at!(env::current_dir())?;
        err_at!(env::set_current_dir(to_dir))?;
        Ok(SwitchDir {
            cwd: cwd.into_os_string(),
            _to_dir: to_dir.to_os_string(),
        })
    }
}
