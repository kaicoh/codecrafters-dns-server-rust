use crate::Result;
use std::io::Read;

pub fn read_1_byte<R: Read>(r: &mut R) -> Result<u8> {
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf)?;
    Ok(buf[0])
}

pub fn read_2_bytes<R: Read>(r: &mut R) -> Result<[u8; 2]> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

pub fn read_4_bytes<R: Read>(r: &mut R) -> Result<[u8; 4]> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

pub fn read_n_bytes<R: Read>(r: &mut R, n: usize) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; n];
    r.read_exact(&mut buf)?;
    Ok(buf)
}
