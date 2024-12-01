# Read input from file
input_file = open("input", "r")
input_data = [int(line) for line in input_file.readlines()]
print(input_data)


def mass_to_fuel(mass):
    fuel = [mass // 3 - 2]
    while fuel[-1] > 0:
        fuel.append(fuel[-1] // 3 - 2)
    fuel.pop(-1)
    return sum(fuel)


fuel_req = []
for data in input_data:
    fuel_req.append(mass_to_fuel(data))

fuel_total = sum(fuel_req)
print("The required amount of fuel is {} units.".format(fuel_total))
