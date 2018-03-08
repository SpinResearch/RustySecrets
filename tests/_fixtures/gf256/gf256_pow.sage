from common import field

for i in sorted(field):
    for j in range(0, 256):
        print("{} ^ {} = {}".format(i, j, i ^ j))
