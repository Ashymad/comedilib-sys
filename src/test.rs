use super::*;
use std::ffi::CString;

#[test]
fn open_device() {
    unsafe {
        let dev = CString::new("/dev/comedi0").unwrap();
        let it = comedi_open(dev.as_ptr());
        if it.is_null() {
            panic!("Couldn't open /dev/comedi0, make sure you have connected comedi device to your computer!");
        };
    }
}
