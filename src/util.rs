use dirs;

use std::{ffi, path};

use crate::{Error, Result};

#[macro_export]
macro_rules! if_else {
    ($pred:expr, $if:expr, $else:expr) => {
        if $pred {
            $if
        } else {
            $else
        }
    };
}

#[macro_export]
macro_rules! limit {
    ($op:expr, $min:expr, $max:expr) => {{
        let res = $op;
        if_else!(res < $min, $min, if_else!(res < $max, res, $max))
    }};
    ($op:expr, $max:expr) => {{
        let res = $op;
        if_else!(res < $max, res, $max)
    }};
}

#[macro_export]
macro_rules! limite {
    ($op:expr, $min:expr, $max:expr) => {{
        let res = $op;
        let max = $max.saturating_sub(1);
        if_else!(res < $min, $min, if_else!(res < $max, res, max))
    }};
    ($op:expr, $max:expr) => {{
        let res = $op;
        let max = $max.saturating_sub(1);
        if_else!(res < $max, res, max)
    }};
}

#[macro_export]
macro_rules! err_at {
    ($v:ident, msg:$m:expr) => {
        //
        Err(Error::$v(format!("{}:{} {}", file!(), line!(), $m)))
    };
    ($v:ident, $e:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let m = format!("{}:{} err:{:?}", file!(), line!(), err);
                Err(Error::$v(m))
            }
        }
    };
    ($v:ident, $e:expr, $m:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let m = format!("{}:{} {} err:{:?}", file!(), line!(), $m, err);
                Err(Error::$v(m))
            }
        }
    };
}

pub fn to_file_loc(file_name: &ffi::OsStr) -> Result<ffi::OsString> {
    let p = path::Path::new(file_name);
    if p.is_relative() {
        let home_dir = err_at!(
            Fatal,
            dirs::home_dir().ok_or(format!("can't find home-directory"))
        )?;
        let f: path::PathBuf = [home_dir, p.to_path_buf()].iter().collect();
        Ok(f.into_os_string())
    } else {
        Ok(file_name.to_os_string())
    }
}
