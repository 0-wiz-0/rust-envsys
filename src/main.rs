use std::error::Error;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use nix::ioctl_readwrite;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use plist::Value;
use std::slice::from_raw_parts;
use std::io::Cursor;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct plistref {
        pref_plist: *mut c_void,               /* plist data */
        pref_len: usize,                /* total length of plist data */
}

ioctl_readwrite!(envsys_getdictionary, b'E', 0, plistref);

fn main() {
    match detect_sensors() {
	Ok(_) => (),
	Err(err) => println!("error: {}", err),
    }	
}

fn detect_sensors() -> Result <(), Box<dyn Error>> {
    let envsys = File::open("/dev/sysmon")?;
    let mut dict = MaybeUninit::<plistref>::uninit();
    let _res = unsafe { envsys_getdictionary(envsys.as_raw_fd(), dict.as_mut_ptr()) };
    let dict = unsafe { dict.assume_init() };
    let u8slice: &[u8] = unsafe { from_raw_parts(dict.pref_plist as *const u8, dict.pref_len) };
    let cursor = Cursor::new(u8slice);
    let value = Value::from_reader(cursor)?;
    println!("{:?}", value);
    Ok(())
}
