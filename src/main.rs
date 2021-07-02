use std::error::Error;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use nix::ioctl_readwrite;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use plist::Value;
use std::slice::from_raw_parts;
use std::io::Cursor;
use std::process;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct plistref {
        pref_plist: *mut c_void,               /* plist data */
        pref_len: usize,                /* total length of plist data */
}

ioctl_readwrite!(envsys_getdictionary, b'E', 0, plistref);

fn main() {
    let envsys_data = detect_sensors().unwrap_or_else(|err| {
	println!("error: {}", err);
	process::exit(1);
    });
    print_envsys(&envsys_data);
}

fn detect_sensors() -> Result <plist::Value, Box<dyn Error>> {
    let envsys = File::open("/dev/sysmon")?;
    let mut dict = MaybeUninit::<plistref>::uninit();
    let _res = unsafe { envsys_getdictionary(envsys.as_raw_fd(), dict.as_mut_ptr()) };
    let dict = unsafe { dict.assume_init() };
    let u8slice: &[u8] = unsafe { from_raw_parts(dict.pref_plist as *const u8, dict.pref_len) };
    let cursor = Cursor::new(u8slice);
    let value = Value::from_reader(cursor)?;
    //println!("{:?}", value);
    Ok(value)
}

fn print_envsys(data: &plist::Value) {
    if let plist::Value::Dictionary(dict) = data {
	for (key, value) in dict {
	    println!("sensor {}", key);
	    if let plist::Value::Array(a) = value {
		for entry in a {
		    if let plist::Value::Dictionary(dict2) = entry {
			if let Some(plist::Value::String(description)) = dict2.get("description") {
			    println!("\tsub-sensor {}", description);
			    for (key2, value2) in dict2 {
				println!("\t\t{} => {:?}", key2, value2);
			    }
			}
			else if let Some(plist::Value::Dictionary(dev)) = dict2.get("device-properties") {
			    println!("\tdevice properties:");
			    for (key2, value2) in dev {
				println!("\t\t{} => {:?}", key2, value2);
			    }
			}
			else {
			    println!("unexpected data type (sensor data level)")
			}
		    }
		    else {
			println!("unexpected data (sensor data level)")
		    }
		}
	    } else {
		println!("unexpected data (sensor level)")
	    }
	}
    } else {
	println!("unexpected data (toplevel)")
    }
}
