use ::ExecPath;
use ::Embed;
use std::io;

/// The module for a compile-time data embedder: AutoEmbed

pub struct AutoEmbed {
    be: Box<Embed>
}

/// `AutoEmbed` is a wrapper embedder that uses an apropreate Embeder at
/// compile time. (Defaults to `GenericEmbed`)
impl AutoEmbed {
    pub fn new(executable: ExecPath) -> io::Result<AutoEmbed> {
        match ::generic::GenericEmbed::new(executable) {
            Ok(e) => Ok(AutoEmbed{be:Box::new(e) as Box<Embed>}),
            Err(e) => Err(e)
        }
    }
}

impl Embed for AutoEmbed {
    fn load(&self) -> io::Result<Vec<u8>> {self.be.load()}
    fn strip(&mut self) -> io::Result<()> {self.be.strip()}
    fn store(&mut self, data: &[u8]) -> io::Result<()> {self.be.store(data)}
}



