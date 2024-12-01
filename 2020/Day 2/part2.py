with open("input") as f:
    program_input = f.readlines()


def check_password(password, letter, prange):
    if (letter == password[prange[0]]) ^ (letter == password[prange[1]]):
        return True
    else:
        return False

counter = 0
for line in program_input:
    split = line.split(":")
    split_policy = split[0].split(" ")
    policy_range = [int(i) for i in split_policy[0].split("-")]
    policy_letter = split_policy[1]
    password = split[1]

    if check_password(password, policy_letter, policy_range):
        counter = counter + 1

print("The number of valid passwords was {}".format(counter))
