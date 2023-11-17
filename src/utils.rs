use crc::{Algorithm, Crc};

/// Calculates the crc
pub fn checksum_ieee(algo: &'static Algorithm<u32>, bytes: &[u8]) -> u32 {
    let crc = Crc::<u32>::new(&algo);
    crc.checksum(bytes)
}
