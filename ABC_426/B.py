s = input()

d = {}
for c in s:
    if c in d:
        d[c] += 1
    else:
        d[c] = 1

for i in d:
    if d[i] == 1:
        print(i)

