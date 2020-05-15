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

    let parsers = vec![Parser {
        name: "txt_en".to_string(),
        dir: "ts/txt_en".into(),
        grammar: "ts/txt_en/grammar.js".into(),
        sources: vec!["ts/txt_en/src/parser.c".into()],
    }];
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
        let _sd = SwitchDir::to_lib_dir(&parser.dir);
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
        b.include(err_at!(env::current_dir())?.join(parser.dir).join("src"));
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

    fn to_lib_dir(to_dir: &ffi::OsStr) -> Result<SwitchDir, String> {
        let cwd = err_at!(env::current_dir())?;
        err_at!(env::set_current_dir(to_dir))?;
        Ok(SwitchDir {
            cwd: cwd.into_os_string(),
            _to_dir: to_dir.to_os_string(),
        })
    }
}