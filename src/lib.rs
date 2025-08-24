#![feature(extern_types)]

mod midl;

#[cfg(all(target_endian = "little", target_os = "linux", target_pointer_width = "64"))]
mod mdb_linux_64bit;
#[cfg(all(target_endian = "little", target_os = "linux", target_pointer_width = "64"))]
pub use mdb_linux_64bit::*;

#[cfg(all(target_endian = "little", target_os = "macos", target_pointer_width = "64"))]
mod mdb_macos_64bit_m1;
#[cfg(all(target_endian = "little", target_os = "macos", target_pointer_width = "64"))]
pub use mdb_macos_64bit_m1::*;
