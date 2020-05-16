pub struct Edit {
    name: String,
    arg: ffi::OsString,
}

impl Edit {
    pub fn new(name: String, arg: ffi::OsString) -> Edit {
        Edit {
            name,
            arg: arg.trim(),
        }
    }
}

impl Edit {
    fn to_name(&self) -> String {
        "edit".to_string()
    }
}
