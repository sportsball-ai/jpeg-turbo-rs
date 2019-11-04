extern crate libjpeg_turbo_sys;

use std::fmt;
use std::os::raw::c_int;

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Copy)]
pub struct PixelFormat(pub libjpeg_turbo_sys::TJPF);

impl PixelFormat {
    pub const FORMAT_RGB: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_RGB);
    pub const FORMAT_BGR: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_BGR);
    pub const FORMAT_RGBX: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_RGBX);
    pub const FORMAT_BGRX: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_BGRX);
    pub const FORMAT_XBGR: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_XBGR);
    pub const FORMAT_XRGB: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_XRGB);
    pub const FORMAT_GRAY: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_GRAY);
    pub const FORMAT_RGBA: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_RGBA);
    pub const FORMAT_BGRA: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_BGRA);
    pub const FORMAT_ABGR: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_ABGR);
    pub const FORMAT_ARGB: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_ARGB);
    pub const FORMAT_CMYK: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_CMYK);
    pub const FORMAT_UNKNOWN: PixelFormat = PixelFormat(libjpeg_turbo_sys::TJPF_TJPF_UNKNOWN);
}

#[derive(Clone, Copy)]
pub struct Sampling(pub libjpeg_turbo_sys::TJSAMP);

impl Sampling {
    pub const SAMPLING_444: Sampling = Sampling(libjpeg_turbo_sys::TJSAMP_TJSAMP_444);
    pub const SAMPLING_422: Sampling = Sampling(libjpeg_turbo_sys::TJSAMP_TJSAMP_422);
    pub const SAMPLING_420: Sampling = Sampling(libjpeg_turbo_sys::TJSAMP_TJSAMP_420);
    pub const SAMPLING_GRAY: Sampling = Sampling(libjpeg_turbo_sys::TJSAMP_TJSAMP_GRAY);
    pub const SAMPLING_440: Sampling = Sampling(libjpeg_turbo_sys::TJSAMP_TJSAMP_440);
    pub const SAMPLING_411: Sampling = Sampling(libjpeg_turbo_sys::TJSAMP_TJSAMP_411);
}

pub fn buf_size(width: i32, height: i32, sampling: Sampling) -> usize {
    unsafe {
        libjpeg_turbo_sys::tjBufSize(width, height, sampling.0 as _) as _
    }
}

pub struct Compressor {
    handle: libjpeg_turbo_sys::tjhandle,
}

impl Compressor {
    pub fn new() -> Compressor {
        Compressor{
            handle: unsafe { libjpeg_turbo_sys::tjInitCompress() },
        }
    }

    pub fn compress(&mut self, pixels: &[u8], width: i32, height: i32, pixel_format: PixelFormat, destination: &mut [u8], sampling: Sampling, quality: i32) -> Result<(), Error> {
        let mut destination_size = destination.len() as u64;
        unsafe {
            self.check(libjpeg_turbo_sys::tjCompress2(
                self.handle,
                pixels.as_ptr(),
                width,
                0,
                height,
                pixel_format.0,
                &mut destination.as_mut_ptr(),
                &mut destination_size,
                sampling.0 as _,
                quality,
                libjpeg_turbo_sys::TJFLAG_NOREALLOC as _,
            ))
        }
    }

    unsafe fn check(&mut self, status: c_int) -> Result<(), Error> {
        match status {
            0 => Ok(()),
            _ => Err(self.last_error()),
        }
    }

    unsafe fn last_error(&mut self) -> Error {
        Error{
            message: std::ffi::CStr::from_ptr(libjpeg_turbo_sys::tjGetErrorStr2(self.handle)).to_str().unwrap_or("unknown error").to_string(),
        }
    }
}

impl Drop for Compressor {
    fn drop(&mut self) {
        unsafe {
            libjpeg_turbo_sys::tjDestroy(self.handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = Compressor::new();
    }
}
