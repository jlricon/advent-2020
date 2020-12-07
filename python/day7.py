import itertools

with open("input/day7.txt", "r") as f:
    lines = f.readlines()
    bag_container = {}
    n = 0
    for line in lines:
        parsed_bags = (
            line.strip()
            .replace(" bags.", "")
            .replace("bag.", "")
            .replace("bags", "")
            .replace("bag", "")
            .split(" contain ")
        )
        other_bags = parsed_bags[1:]
        this_bag = parsed_bags[0].strip()
        bag_to_bag_n = {}
        for other_bag in other_bags[0].split(", "):
            if other_bag == "no other":
                continue
            n_other = int(other_bag[0])
            name_other = other_bag[1:].strip()
            bag_to_bag_n[name_other] = n_other
        bag_container[this_bag.strip()] = bag_to_bag_n


def colors_contained_by_one_color(this_color) -> set[str]:
    if this_color not in bag_container:
        return set()
    contained_bags = bag_container[this_color]
    return set(contained_bags) | set(
        itertools.chain(*[colors_contained_by_one_color(x) for x in contained_bags])
    )


def bags_inside_one_bag(color) -> int:
    if color not in bag_container:
        return 0
    else:
        contained_bags = sum(bag_container[color].values())

        return contained_bags + sum(
            [
                n_contained * bags_inside_one_bag(contained_color)
                for contained_color, n_contained in bag_container[color].items()
            ]
        )


all_colors = bag_container.keys()
all_bags_colors = map(
    lambda color: (color, colors_contained_by_one_color(color)), all_colors
)
colors_with_gold = filter(
    lambda color_container: "shiny gold" in color_container[1], all_bags_colors
)

res = len(list(colors_with_gold))
print(res)
print(bags_inside_one_bag("shiny gold"))
