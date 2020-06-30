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
    ($e:expr) => {{
        use Error::{BadPattern, Fatal, IOError, Invalid, NoTopic, IPC};
        use Error::{FailBuffer, FailConvert, FailParse};

        let prefix = format!("{}:{}", file!(), line!());
        match $e {
            Ok(val) => Ok(val),
            Err(Fatal(s)) => Err(Fatal(format!("{} {}", prefix, s))),
            Err(BadPattern(s)) => Err(BadPattern(format!("{} {}", prefix, s))),
            Err(IOError(s)) => Err(IOError(format!("{} {}", prefix, s))),
            Err(IPC(s)) => Err(IPC(format!("{} {}", prefix, s))),
            Err(NoTopic) => Err(NoTopic),
            Err(Invalid(s)) => Err(Invalid(format!("{} {}", prefix, s))),
            Err(FailConvert(s)) => Err(FailConvert(format!("{} {}", prefix, s))),
            Err(FailParse(s)) => Err(FailParse(format!("{} {}", prefix, s))),
            Err(FailBuffer(s)) => Err(FailBuffer(format!("{} {}", prefix, s))),
        }
    }};
    ($v:ident, msg:$m:expr) => {
        //
        Err(Error::$v(format!("{}:{} {}", file!(), line!(), $m)))
    };
    ($v:ident, $e:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let m = format!("{}:{} {}", file!(), line!(), err);
                Err(Error::$v(m))
            }
        }
    };
    ($v:ident, $e:expr, $m:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let m = format!("{}:{} {} {}", file!(), line!(), $m, err);
                Err(Error::$v(m))
            }
        }
    };
}
