Given a working header, the rest of the file will contain one bit for each voxel. A 1 signifies there is something there, a 0 there is not. For example, all 0s will be a blank space, while all 1s will be a cuboid covering the entire defined size.

Let's take an example 4x4x4 RWVXLA file, 2 frames, without the header, displayed in binary, and track the X, Y, Z and F for each bit:
`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`F 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`


`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 1 1 1 1 1 1 1 1   1 1 1 1 1 1 1 1`

`F 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`


`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 2 2 2 2 2 2 2 2   2 2 2 2 2 2 2 2`

`F 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`


`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 3 3 3 3 3 3 3 3   3 3 3 3 3 3 3 3`

`F 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`F 1 1 1 1 1 1 1 1   1 1 1 1 1 1 1 1`


`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 1 1 1 1 1 1 1 1   1 1 1 1 1 1 1 1`

`F 1 1 1 1 1 1 1 1   1 1 1 1 1 1 1 1`


`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 2 2 2 2 2 2 2 2   2 2 2 2 2 2 2 2`

`F 1 1 1 1 1 1 1 1   1 1 1 1 1 1 1 1`


`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 3 3 3 3 3 3 3 3   3 3 3 3 3 3 3 3`

`F 1 1 1 1 1 1 1 1   1 1 1 1 1 1 1 1`


As we can see, the X is incremented with each bit, the Y with each reset of X, the Z with each reset of Y, and the frame counter after each frame is done. At the end of the model, the file simply ends. No footer needed.