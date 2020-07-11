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

        let p = format!("{}:{}", file!(), line!());
        match $e {
            Ok(val) => Ok(val),
            Err(Fatal(_, s)) => Err(Fatal(p, s)),
            Err(BadPattern(_, s)) => Err(BadPattern(p, s)),
            Err(IOError(_, s)) => Err(IOError(p, s)),
            Err(IPC(_, s)) => Err(IPC(p, s)),
            Err(NoTopic(_)) => Err(NoTopic(p)),
            Err(Invalid(_, s)) => Err(Invalid(p, s)),
            Err(FailConvert(_, s)) => Err(FailConvert(p, s)),
            Err(FailParse(_, s)) => Err(FailParse(p, s)),
            Err(FailBuffer(_, s)) => Err(FailBuffer(p, s)),
        }
    }};
    ($v:ident, msg:$m:expr) => {{
        let prefix = format!("{}:{}", file!(), line!());
        Err(Error::$v(prefix, format!("{}", $m)))
    }};
    ($v:ident, $e:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                Err(Error::$v(prefix, format!("{}", err)))
            }
        }
    };
    ($v:ident, $e:expr, $m:expr) => {
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                Err(Error::$v(prefix, format!("{} {}", $m, err)))
            }
        }
    };
}
