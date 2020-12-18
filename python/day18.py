import re
class Ops(int):
    def __mul__(self,x):
        return Ops(int(self) +x)
    def __add__(self,x):
        return Ops(int(self) +x)
    def __sub__(self,x):
        return Ops(int(self) *x)

def operate(expr, pt2=False):
    expr = re.sub(r"(\d+)", r"Ops(\1)", expr)
    expr = expr.replace("*", "-")
    if pt2:
        expr = expr.replace("+", "*")
    return eval(expr, {}, {"Ops": Ops})

with open('input/day18.txt') as f:
    lines = f.read().splitlines()

print("Part 1:", sum(operate(l) for l in lines))
print("Part 2:", sum(operate(l, pt2=True) for l in lines))