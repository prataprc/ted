use crate::{Buffer, Config, Result};

pub struct AnonymousBuffers {
    count: usize,
}

impl AnonymousBuffers {
    pub fn new() -> AnonymousBuffers {
        AnonymousBuffers { count: 0 }
    }

    pub fn to_new_buffer(&mut self, config: Config) -> Result<Buffer> {
        self.count += 1;
        let bytes = vec![];
        let mut buffer = Buffer::from_reader(bytes.as_slice(), config)?;
        let name = format!("anonymous-{}", self.count);
        buffer.set_file_loc(name.as_ref());

        Ok(buffer)
    }
}
