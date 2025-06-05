// imports
use std::fs;
// used for file handling
use std::str; // used for handling of the name
use bitvec::prelude::*; // used for handling of the voxel data


// object for a static voxel
#[derive(Debug)]
pub struct VoxelObject {
    pub name: String,
    pub data: VoxelData,
}

// object for an animated voxel, where each frame is a VoxelData object
pub struct AnimatedVoxelObject {
    pub name: String,
    pub frames: Vec<VoxelData>,
}

// struct to contain raw voxel data
#[derive(Debug)]
pub struct VoxelData {
    pub size: (u32, u32, u32),
    pub voxels: Vec<Vec<BitVec<u8, Msb0>>>,
}

// this feels pointless but sure i guess
#[derive(Clone)]
enum StrOrVecOrInt32OrBool {
    Str(String),
    Vec(Vec<u8>),
    T32((u32, u32, u32)),
    Int32(i32),
    Bool(bool),
}

impl StrOrVecOrInt32OrBool {
    fn into_string(self) -> Option<String> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }
    fn into_vec(self) -> Option<Vec<u8>> {
        match self {
            Self::Vec(v) => Some(v),
            _ => None,
        }
    }
    fn into_t32(self) -> Option<(u32, u32, u32)> {
        match self {
            Self::T32(t) => Some(t),
            _ => None,
        }
    }
    fn into_int32(self) -> Option<i32> {
        match self {
            Self::Int32(i) => Some(i),
            _ => None,
        }
    }
    fn into_bool(self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(b),
            _ => None,
        }
    }
}

fn process_file(path: &str) -> Vec<StrOrVecOrInt32OrBool> {
    let file_contents = fs::read(path)
        .expect("Failed to read file");
    // First 8 bytes of file are the name
    let name = str::from_utf8(&file_contents[0..8].to_vec())
        .expect("Name is invalid UTF-8")
        .to_string();
    // Bytes 17-24 are the size of the voxel object
    let size = (
        u32::from_le_bytes(file_contents[16..20].try_into().expect("Invalid size")),
        u32::from_le_bytes(file_contents[20..24].try_into().expect("Invalid size")),
        u32::from_le_bytes(file_contents[24..28].try_into().expect("Invalid size")),
    );
    let version = i32::from_le_bytes(file_contents[28..32].try_into().expect("Invalid version"));
    let animated = file_contents[13] == 65;
    vec![StrOrVecOrInt32OrBool::Vec(file_contents),
        StrOrVecOrInt32OrBool::Str(name),
        StrOrVecOrInt32OrBool::Bool(animated),
        StrOrVecOrInt32OrBool::Int32(version),
        StrOrVecOrInt32OrBool::T32(size)]
}

fn process_frame(frame_num: u32, voxels_raw: &BitSlice<u8, Msb0>, size: (u32, u32, u32)) -> VoxelData {
    let frame_size = size.0 * size.1 * size.2;
    let frame: &BitSlice<u8, Msb0> = &voxels_raw[(frame_num * frame_size as u32) as usize..((frame_num + 1) * frame_size as u32) as usize];
    let mut voxels: Vec<Vec<BitVec<u8, Msb0>>> = Vec::new();
    for plane in 0..size.2 {
        let mut plane_voxels: Vec<BitVec<u8, Msb0>> = Vec::new();
        for row in 0..size.1 {
            let segment_idx = (plane * size.2 + row) as usize;
            let start = segment_idx * size.0 as usize;
            let end = start + size.0 as usize;
            let segment: BitVec<u8, Msb0> = frame[start..end].to_bitvec();
            plane_voxels.push(segment);
        }
        voxels.push(plane_voxels);}
    VoxelData {
        size,
        voxels,
    }
}

// importer for static voxel objects
pub fn import_rwvxl(path: &str) -> VoxelObject {
    let file_processed_vec = process_file(path);
    let file_contents = file_processed_vec[0].clone().into_vec().expect("File contents are not a Vec<u8>");
    let size = file_processed_vec[4].clone().into_t32().expect("Size is not a (u32, u32, u32)");
    let name = file_processed_vec[1].clone().into_string().expect("Name is not a String");
    println!("Size: {}x{}x{}", size.0, size.1, size.2);
    let voxels_raw: &BitSlice<u8, Msb0> = file_contents[32..].view_bits::<Msb0>();
    VoxelObject {
        name,
        data: process_frame(0, voxels_raw, size),
    }
}