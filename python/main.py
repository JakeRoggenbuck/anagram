words = {}

with open("../wordlist.txt") as file:
    for x in file.readlines():
        h = hash(''.join(sorted(x)))

        if h in words:
            words[h].append(x)
        else:
            words[h] = [x]

count = 0
for k, v in words.items():
    if len(v) > 1:
        count += len(v)

print(count)
