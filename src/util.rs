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
