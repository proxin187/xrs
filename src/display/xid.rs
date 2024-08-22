use super::*;

// NOTE: xcb has a fucked up implementation of this, never using the xcb source as an example again


pub struct Xid {
    base: u32,
    mask: u32,
    next: u32,
}

impl Xid {
    pub fn new() -> Xid {
        Xid {
            base: 0,
            mask: 0,
            next: 0,
        }
    }

    pub fn next(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        self.next += 1;

        if self.next >= self.mask {
            Err(Box::new(Error::RanOutOfXid))
        } else {
            Ok(self.next | self.base)
        }
    }
}


