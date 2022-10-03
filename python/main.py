words = {}

with open("../wordlist.txt") as file:
    for x in file.readlines():
        h = hash(''.join(sorted(x)))

        if h in words:
            words[h].append(x)
        else:
            words[h] = [x]
