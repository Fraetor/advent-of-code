with open("input") as f:
    tree_map = f.readlines()

tree_map = [[char for char in line if char == "#" or char == "."] for line in tree_map]


def check_slope(tree_map, right, down):
    coords = [0, 0]
    tree_count = 0
    while True:
        #print(coords)
        if coords[0] > len(tree_map)-1:
            break
        if tree_map[coords[0]][coords[1]] == "#":
            tree_count = tree_count + 1
        coords = [coords[0] + down, coords[1] + right]
        if coords[1] > len(tree_map[0])-1:
            coords[1] = coords[1] - len(tree_map[0])
    print("{} trees hit going right {} and down {}.".format(tree_count, right, down))
    return tree_count


product = check_slope(tree_map, 1, 1) \
        * check_slope(tree_map, 3, 1) \
        * check_slope(tree_map, 5, 1) \
        * check_slope(tree_map, 7, 1) \
        * check_slope(tree_map, 1, 2)

print(product)
