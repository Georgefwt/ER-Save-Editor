use std::io::Error;
use binary_reader::BinaryReader;
use crate::write::write::Write;
use crate::read::read::Read;

/// Size of the regulation region, i.e. everything in UserData11 after `unk`.
/// The regulation blob itself only takes up part of it and the remainder is
/// padding; the blob grows with every game patch, so we keep the region whole
/// instead of hardcoding where the blob ends. Both the decrypt and the DCX
/// decompress step read the length they need out of the blob's own header and
/// ignore the padding that follows.
const REGULATION_REGION_SIZE: usize = 0x240000;

pub struct UserData11 {
    unk: [u8;0x10],
    pub regulation: Vec<u8>,
}

impl Default for UserData11 {
    fn default() -> Self {
        Self { 
            unk: Default::default(), 
            regulation: vec![0; REGULATION_REGION_SIZE],
        }
    }
}

impl Read for UserData11 {
    fn read(br: &mut BinaryReader) -> Result<UserData11, Error> {
        let mut user_data_11 = UserData11::default();
        user_data_11.unk.copy_from_slice(br.read_bytes(0x10)?);
        user_data_11.regulation.copy_from_slice(br.read_bytes(REGULATION_REGION_SIZE)?);
        Ok(user_data_11)
    }
}

impl Write for UserData11 {
    fn write(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.unk);
        bytes.extend(self.regulation.to_vec());
        Ok(bytes)
    }
}