`rwvxl-utils` is my poor attempt at making a rust library (never done this before, could you tell?) to handle RWVXL and RWVXLA files. It stores each row of pixels in a BitVec, which is then placed in a vector with other BitVecs to create a plane, which is then placed in a vector with other plane vectors to create the final structure. Simple, right?

Currently all main.rs does is print the entire contents of my 64x64x64 blank test file (manually made in a hex editor) though I plan on making it a bit better later on.

Future plans involve adding render support (rendering these voxels into a visual image was my main motivation for making this), exporting to other voxel file formats, importing from other voxel formats, adding new file versions with support for variable bit depth colour (this is why I've got the version field, so any breaking change gets a new number there, and all code should know the type of RWVXL it's dealing with)
