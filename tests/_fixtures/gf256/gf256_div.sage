from common import field

for i in sorted(field):
    for j in sorted(field)[1:]:
        print("{} / {} = {}".format(i, j, i / j))
