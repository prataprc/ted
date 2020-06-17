use crate::{buffer::Buffer, color_scheme::ColorScheme, tss::Automata};

struct Syntax {
    buf: Buffer,
    atm: Automata,
    scheme: ColorScheme,
}

impl Syntax {
    fn new(buf: Buffer) -> Syntax {
        Syntax {
            buf,
            atm: Default::default(),
            scheme: Default::default(),
        }
    }

    fn set_color_scheme(&mut self, scheme: ColorScheme) -> &mut Self {
        self.scheme = scheme;
        self
    }

    fn set_style_sheet(&mut self, text: &str) -> &mut Self {
        self.atm = Automata::from_str(text, &self.scheme);
        self
    }
}
