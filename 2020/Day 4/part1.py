def get_passports(data):
    passports = []
    for line in data.split("\n\n"):
        if line == ['']:
            break
        pair_list = []
        pairs = line.replace("\n", " ").split(" ")
        for pair in pairs:
            pair_list.append(pair.split(":"))
        #print(pair_list)
        passport = dict(pair_list)
        #print(passport)
        passports.append(passport)
    return passports


with open("input") as f:
    data = f.read()

count = 0
for passport in get_passports(data):
    fields = sorted(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"])
    npc = sorted(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
    if sorted(passport) == fields or sorted(passport) == npc:
        count = count + 1

print("{} valid passsports".format(count))
