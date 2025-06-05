// imports
use std::fs; // used for basic file handling
use std::str; // used for handling of the name
use bitvec::prelude::*; // used for handling of the voxel data


// object for a static voxel
#[derive(Debug)]
pub struct VoxelObject {
    pub name: String, // this is the only metadata in the format as of right now
    pub data: VoxelData, // all data about the voxel model itself is in a separate object
}

// object for an animated voxel, where each frame is a VoxelData object
pub struct AnimatedVoxelObject {
    pub name: String, // same as VoxelObject
    pub framecount: u16, // makes things easier, might as well since it's specified in the file format
    pub frames: Vec<VoxelData>, // store each frame as a VoxelData object in a vector
}

// struct to contain raw voxel data
#[derive(Debug)]
pub struct VoxelData {
    pub size: (u32, u32, u32), // size of the image in voxels (width, height, depth)
    pub voxels: Vec<Vec<BitVec<u8, Msb0>>>, // actual raw image data, docs coming soon
}

// create multitype enum to return a vec for different types later
#[derive(Clone)]
enum MultiType {
    Str(String),
    Vec(Vec<u8>),
    T32((u32, u32, u32)),
    Int32(i32),
    Bool(bool),
}

// impl MultiType to return the direct type for each case
// there should be a cleaner way for this but i haven't found one yet
impl MultiType {
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

// extract file contents and metadata for ease of use later
fn process_file(path: &str) -> Vec<MultiType> {
    let file_contents = fs::read(path) // grab the raw bytes of the file
        .expect("Failed to read file");
    // First 8 bytes of file are the name
    let name = str::from_utf8(&file_contents[0..8].to_vec()) // get the name out of it
        .expect("Name is invalid UTF-8")
        .to_string();
    // Bytes 17-24 are the size values so we know how big a voxel is
    let size = (
        u32::from_le_bytes(file_contents[16..20].try_into().expect("Invalid size")),
        u32::from_le_bytes(file_contents[20..24].try_into().expect("Invalid size")),
        u32::from_le_bytes(file_contents[24..28].try_into().expect("Invalid size")),
    ); // each is store in the file as u32
    let version = i32::from_le_bytes(file_contents[28..32].try_into().expect("Invalid version")); // used to check compatibility - useless until initial spec is finished
    let animated = file_contents[13] == 65; // check if the UTF-8 filetype id is "RWRVXL" or "RWRVXLA" by checking if the A is present
    vec![MultiType::Vec(file_contents),
        MultiType::Str(name),
        MultiType::Bool(animated),
        MultiType::Int32(version),
        MultiType::T32(size)] // return all useful values in a vector for processing by other functions
}

// process a single frame of voxel data
fn process_frame(frame_num: u16, voxels_raw: &BitSlice<u8, Msb0>, size: (u32, u32, u32)) -> VoxelData {
    let frame_size = size.0 * size.1 * size.2; // calculate size of one frame
    let frame: &BitSlice<u8, Msb0> = &voxels_raw[(frame_num as u32 * frame_size as u32) as usize..((frame_num as u32 + 1) * frame_size as u32) as usize]; // only process the frame requested
    let mut voxels: Vec<Vec<BitVec<u8, Msb0>>> = Vec::new(); // create final voxels vector to hold the processed data
    for plane in 0..size.2 {
        let mut plane_voxels: Vec<BitVec<u8, Msb0>> = Vec::new(); // create a temporary inner vector to hold the voxels for this plane
        for row in 0..size.1 {
            // process a single row of voxels on the plane into a bitvec
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
    } // return VoxelData object directly for ease of use
}

// importer for static voxel objects
pub fn import_rwvxl(path: &str) -> VoxelObject {
    // call processing function and extract needed data
    let file_processed_vec = process_file(path);
    let file_contents = file_processed_vec[0].clone().into_vec().expect("File contents are not a Vec<u8>");
    let size = file_processed_vec[4].clone().into_t32().expect("Size is not a (u32, u32, u32)");
    let name = file_processed_vec[1].clone().into_string().expect("Name is not a String");
    if file_processed_vec[2].clone().into_bool().expect("Animated flag is not a bool") {
        panic!("This is an animated voxel file, use import_rwvxla instead."); // things can go very wrong if this check is not done
    }
    let voxels_raw: &BitSlice<u8, Msb0> = file_contents[32..].view_bits::<Msb0>(); // due to weirdness with pointers i'm yet to understand, we can't do this in process_file
    VoxelObject {
        name,
        data: process_frame(0, voxels_raw, size), // frame 0 is the only frame in a static voxel object
    }
}

// importer for animated voxel objects
pub fn import_rwvxla(path: &str) -> AnimatedVoxelObject {
    // call processing function and extract needed data
    let file_processed_vec = process_file(path);
    let file_contents = file_processed_vec[0].clone().into_vec().expect("File contents are not a Vec<u8>");
    let size = file_processed_vec[4].clone().into_t32().expect("Size is not a (u32, u32, u32)");
    let name = file_processed_vec[1].clone().into_string().expect("Name is not a String");
    let framecount = file_processed_vec[3].clone().into_int32().expect("Frame count is not an i32") as u16; // convert to u16 for easier handling
    if !file_processed_vec[2].clone().into_bool().expect("Animated flag is not a bool") {
        panic!("This is a static voxel file, use import_rwvxl instead."); // things can go very wrong if this check is not done
    }
    let voxels_raw: &BitSlice<u8, Msb0> = file_contents[32..].view_bits::<Msb0>(); // due to weirdness with pointers i'm yet to understand, we can't do this in process_file
    let mut frames: Vec<VoxelData> = Vec::new(); // create a vector to hold all frames
    for frame in 0..framecount {
        frames.push(process_frame(frame, voxels_raw, size)); // process each frame and push it to the vector
    }
    AnimatedVoxelObject {
        name,
        framecount,
        frames,
    }
}