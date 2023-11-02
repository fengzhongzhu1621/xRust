use std::io;

pub struct Buffer(pub Vec<u8>);

impl Buffer {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.extend(buf);
        Ok(buf.len())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}
