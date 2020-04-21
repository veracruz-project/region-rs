use std::io;
use {Region, Protection, Error, Result};

pub fn page_size() -> usize {
  4096
}

pub fn set_protection(_base: *const u8, _size: usize, _protection: Protection) -> Result<()> {
  Ok(())
}

pub fn lock(_base: *const u8, _size: usize) -> Result<()> {
  Ok(())
}

pub fn unlock(_base: *const u8, _size: usize) -> Result<()> {
  Ok(())
}

pub fn get_region(_address: *const u8) -> Result<Region> {
  Err(Error::SystemCall(io::Error::new(io::ErrorKind::Other, "[region::os::icecap::get_region] unsupported")))
}
