with open("input") as f:
    program_input = [int(i) for i in f.readlines()]
    #print(program_input)

for i in program_input:
    others = program_input
    others.remove(i)
    for o in others:
        if i + o == 2020:
            print(i * o)
