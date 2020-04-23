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
macro_rules! bounded_num_op {
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
macro_rules! err_at {
    ($v:ident, msg:$msg:expr) => {
        //
        Err(Error::$v(format!("{}:{} {}", file!(), line!(), $msg)))
    };
    ($v:ident, $e:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let msg = format!("{}:{} err:{}", file!(), line!(), err);
                Err(Error::$v(msg))
            }
        }
    };
    ($v:ident, $e:expr, $msg:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let msg = format!("{}:{} {} err:{}", file!(), line!(), $msg, err);
                Err(Error::$v(msg))
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
