from itertools import combinations
import math

with open("input/day1.txt", "r") as f:
    lines = f.readlines()
    stripped = list(map(lambda x: int(x.strip()), lines))
    for combi in combinations(stripped, 2):
        if sum(combi) == 2020:
            print(math.prod(combi))
            break

    for combi2 in combinations(stripped, 3):
        if sum(combi2) == 2020:
            print(math.prod(combi2))
            break
