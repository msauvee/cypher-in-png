use crate::chunk_type::ChunkType;

pub fn compute(chunk_type: &ChunkType, data: &Vec<u8>) -> u32 {
    let hashing_data = [chunk_type.bytes().as_slice(), data.as_slice()].concat();
    crc::crc32::checksum_ieee(&hashing_data)
}