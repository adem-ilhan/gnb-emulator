pub struct GtpHeader {
    pub flags: u8,
    pub msg_type: u8,
    pub msg_len: u16,
    pub teid: u32,
    // pub seq: u16,
}

impl GtpHeader {
    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.push(self.flags);
        res.push(self.msg_type);
        res.append(&mut self.msg_len.to_be_bytes().to_vec());
        res.append(&mut self.teid.to_be_bytes().to_vec());
        //4,5,6,7 about teid;
        res
    }

    pub fn new() -> GtpHeader {
        let mut header: GtpHeader = GtpHeader {
            flags: 48,
            msg_type: 255,
            msg_len: 0,
            teid: 0,
        };
        header
    }
}
