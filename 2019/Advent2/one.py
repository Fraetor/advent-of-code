input_file = open("input", "r")
default_memory = [int(i) for i in input_file.read().split(",")]
input_file.close()


def opcode1(a, b, c):  # Add
    memory[c] = memory[a] + memory[b]


def opcode2(a, b, c):  # Multiply
    memory[c] = memory[a] * memory[b]


def opcode99():  # Exit
    pass
    #print(",".join(str(integer) for integer in memory))


def interpreter():
    pointer = 0
    while True:
        #  print(",".join(str(integer) for integer in memory))
        if memory[pointer] == 1:
            opcode1(memory[pointer + 1], memory[pointer + 2], memory[pointer + 3])
            pointer += 4
        elif memory[pointer] == 2:
            opcode2(memory[pointer + 1], memory[pointer + 2], memory[pointer + 3])
            pointer += 4
        elif memory[pointer] == 99:
            opcode99()
            break
        else:
            print("Invalid opcode encountered at position {}.".format(pointer))
            exit(1)


# Size of range to search
bound = 100
# Target for program
target = 19690720

for x in range(bound):
    for y in range(bound):
        memory = default_memory[:]
        memory[1], memory[2] = x, y
        interpreter()

        output = memory[0]
        if output == target:
            print("TRUE for X:{} and Y:{}".format(x, y))
