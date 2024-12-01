input_file = open("input", "r")
wire1_input = input_file.readline().split(",")
wire2_input = input_file.readline().split(",")
input_file.close()

#  [(X, Y)], with up and right being positive.
wire1, wire2 = [(0, 0)], [(0, 0)]

for instr in wire1_input:
    direction = instr[0]
    magnitude = int(instr[1:])
    if direction == "U":
        for i in range(magnitude):
            wire1.append((wire1[-1][0], wire1[-1][1] + 1))
    elif direction == "D":
        for i in range(magnitude):
             wire1.append((wire1[-1][0], wire1[-1][1] - 1))
    elif direction == "R":
        for i in range(magnitude):
            wire1.append((wire1[-1][0] + 1, wire1[-1][1]))
    elif direction == "L":
        for i in range(magnitude):
            wire1.append((wire1[-1][0] - 1, wire1[-1][1]))
    else:
        print("Invalid instruction: {}".format(instr))
        exit(1)

for instr in wire2_input:
    direction = instr[0]
    magnitude = int(instr[1:])
    if direction == "U":
        for i in range(magnitude):
            wire2.append((wire2[-1][0], wire2[-1][1] + 1))
    elif direction == "D":
        for i in range(magnitude):
            wire2.append((wire2[-1][0], wire2[-1][1] - 1))
    elif direction == "R":
        for i in range(magnitude):
            wire2.append((wire2[-1][0] + 1, wire2[-1][1]))
    elif direction == "L":
        for i in range(magnitude):
            wire2.append((wire2[-1][0] - 1, wire2[-1][1]))
    else:
        print("Invalid instruction: {}".format(instr))
        exit(1)

crosses = [coords for coords in wire1 if coords in wire2]
crosses.pop(0)
#print(crosses)
#crosses = [(1668, 321), (2551, 376), (2307, 376), (2117, 1100), (2136, 1100), (2155, 1368), (2155, 1544), (2584, 2085), (2584, 1932), (81, 1902), (239, 2369), (495, 2576), (694, 2576), (46, 2483), (239, 1950), (950, 2640), (2970, 1544), (2970, 1312), (2792, 1208), (2250, 1368), (2818, 1513), (3031, 1513), (3058, 1513), (3062, 1513), (2742, 1893), (2742, 1544), (2117, 1376), (2136, 623), (2330, 785), (2792, 1064), (3031, 1064), (3050, 1312), (3050, 1544), (3050, 1603), (3994, 882), (3221, 716), (3058, 716), (2791, 785), (3031, 827), (3058, 827), (3221, 672), (3604, 1245), (4508, 702), (4906, 1011)]


def taxi_distance(coords1, coords2):
    return abs(coords1[0] - coords2[0]) + abs(coords1[1] - coords2[1])


min_distance = min([taxi_distance((0, 0), coords) for coords in crosses])
print("This minimum distance is: {}".format(min_distance))

wire_length = [wire1.index(coords) + wire2.index(coords) for coords in crosses]

shortest = min(wire_length)

print("The minimum length of wire, however, is: {}".format(shortest))
