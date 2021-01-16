use super::*;
use std::ffi::CString;

#[test]
fn open_device_and_read_data() {
    unsafe {
        let dev = CString::new("/dev/comedi0").unwrap();
        let it = comedi_open(dev.as_ptr());
        if it.is_null() {
            panic!("Couldn't open /dev/comedi0, make sure you have connected comedi device to your computer and you have proper rights!");
        };
        let subdev = 0;
        let channel = 0;
        let range = 0;
        let aref = AREF_GROUND;
        let mut data: lsampl_t = 0;
        let retval = comedi_data_read(it, subdev, channel, range, aref, &mut data as *mut u32);
        if retval < 0 {
            panic!("Unable to read data from device!");
        }
    }
}
