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


def check_between(numstring, minimum, maximum):
    n = int(numstring)
    if n >= minimum and n <= maximum:
        return True
    else:
        return False


def is_int(intstring):
    try:
        int(intstring)
        return True
    except ValueError:
        return False


def is_colour_code(colourstring):
    if colourstring[0] == '#':
        try:
            int(colourstring[1:], 16)
            return True
        except ValueError:
            return False
    else:
        return False


def check_height(heightstring):
    if heightstring[-2:] == "cm":
        if check_between(heightstring[:-2], 150, 193):
            return True
    elif heightstring[-2:] == "in":
        if check_between(heightstring[:-2], 59, 76):
            return True
    return False


def validate(passport):
    fields = sorted(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"])
    npc = sorted(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
    if sorted(passport) == fields or sorted(passport) == npc:
        if check_between(passport["byr"], 1920, 2002):
            if check_between(passport["iyr"], 2010, 2020):
                if check_between(passport["eyr"], 2020, 2030):
                    if passport["ecl"] in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]:
                        if is_colour_code(passport["hcl"]):
                            if len(passport["pid"]) == 9 and is_int(passport["pid"]):
                                if check_height(passport["hgt"]):
                                    return True
    return False


with open("input") as f:
    data = f.read()

count = 0
for passport in get_passports(data):
    if validate(passport):
        count = count + 1

print("{} valid passsports".format(count))
