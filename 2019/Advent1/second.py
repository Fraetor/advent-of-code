initial_mass = 144475
fuel = [initial_mass // 3 - 2]
while fuel[-1] > 0:
    fuel.append(fuel[-1] // 3 - 2)
fuel.pop(-1)
print(fuel)
print(sum(fuel))
