32 bytes, represented here as hex
`45 78 61 6D 70 6C 65 20 52 57 56 58 4C 00 00 00`
`Name------------------- Format ID--------      `
`00 00 00 40 00 00 00 40 00 00 00 40 00 00 00 01`
`Length----- Width------ Height----- Version----` 

Name is a UTF-8 string
Format ID is always "RWVXL" for standard RWVXL files
length/width/height are int32
version is also int32 and increased whenever the format changes significantly, used to check compatibility

