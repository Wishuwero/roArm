import json
import sys

import numpy as np

v1 = np.array(
    [0, 5, 0]
    )
v2 = np.array(

    [-3, 5, 0.3]
)
v3 = np.array(

    [1, 3, 0.2]
        )
v4 = np.array(

    [-2, 2, 12]
        )

def getNewVec(vec1, vec2, perc):
    mult = 1 - perc
    return np.add(np.multiply(perc, vec1),np.multiply(mult, vec2))

input = sys.argv[1]
coor = json.loads(input)
vIn = np.array([coor.get("x"), coor.get("y"), coor.get("z")])

xPerc = 0.2
yPerc = 0.4

leftVec = getNewVec(v2, v4, yPerc)
rightVec = getNewVec(v1, v3, yPerc)
finalVec = getNewVec(leftVec, rightVec, xPerc)

print(leftVec)
print(rightVec)
print(finalVec)
