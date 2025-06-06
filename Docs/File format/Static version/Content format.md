Given a working header, the rest of the file will contain one bit for each voxel. A 1 signifies there is something there, a 0 there is not. For example, all 0s will be a blank space, while all 1s will be a cuboid covering the entire defined size.

Let's take an example 4x4x4 RWVXL file, without the header, displayed in binary, and track the X, Y, and Z for each bit:
`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`


`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 1 1 1 1 1 1 1 1   1 1 1 1 1 1 1 1`


`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 2 2 2 2 2 2 2 2   2 2 2 2 2 2 2 2`


`B 0 0 0 0 0 0 0 0   0 0 0 0 0 0 0 0`

`X 0 1 2 3 0 1 2 3   0 1 2 3 0 1 2 3`

`Y 0 0 0 0 1 1 1 1   2 2 2 2 3 3 3 3`

`Z 3 3 3 3 3 3 3 3   3 3 3 3 3 3 3 3`


As we can see, the X is incremented with each bit, the Y with each reset of X, the Z with each reset of Y. At the end of the model, the file simply ends. No footer needed.