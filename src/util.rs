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
