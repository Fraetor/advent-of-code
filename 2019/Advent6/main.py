input_file = open("input", "r")
orbit_data = [[orbiter.rstrip("\n") for orbiter in line.split(")")] for line in input_file.readlines()]


def num_orbits(current_orbit):
    com_distance = 1
    while current_orbit[0] != "COM":
        com_distance += 1
        current_orbit = search_suborbit(current_orbit[0])
    return com_distance


def search_suborbit(target):
    for suborbit in orbit_data:
        if target == suborbit[1]:
            return suborbit


def transfers_to_com(current_orbit):
    com_transfers = ["COM"]
    while current_orbit[0] != "COM":
        com_transfers.append(current_orbit[0])
        current_orbit = search_suborbit(current_orbit[0])
    return com_transfers


total = 0
for orbit in orbit_data:
    total += num_orbits(orbit)

print("The total number of direct and indirect orbits is: {}".format(total))

you_transfers = transfers_to_com(search_suborbit("YOU"))
santa_transfers = transfers_to_com(search_suborbit("SAN"))

transfer_bodies = [you for you in you_transfers if you not in santa_transfers] +\
                  [santa for santa in santa_transfers if santa not in you_transfers]

print("The number of orbital transfers for you to reach Santa is: {}".format(len(transfer_bodies)))
