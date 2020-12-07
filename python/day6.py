from itertools import combinations
import math

with open("input/day6.txt", "r") as f:
    read = f.read().split("\n\n")
    groups = [[set(i) for i in group.split()] for group in read]
    # Part 1
    total = sum(map(lambda sets: len(set.union(*sets)), groups))
    print(total)
    # Part 2
    total = sum(map(lambda sets: len(set.intersection(*sets)), groups))
    print(total)