#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ListRef {
    data: *const u8,
    len: usize,
}

impl From<Vec<u8>> for ListRef {
    fn from(v: Vec<u8>) -> Self {
        Self::from(v.as_slice())
    }
}

impl From<&Vec<u8>> for ListRef {
    fn from(v: &Vec<u8>) -> Self {
        Self::from(v.as_slice())
    }
}

impl From<&[u8]> for ListRef {
    fn from(v: &[u8]) -> Self {
        let len = v.len();
        let data = v.as_ptr().cast();
        ListRef { data, len }
    }
}
