s = input()

t = ["."] * len(s)
for i in range(len(s)):
    if s[i] == "#":
        t[i] = "#"

f = True
for i in range(len(s)):
    if s[i] == "#":
        continue
    if f or s[i-1] == "#":
        t[i] = "o"
        f = False

print(*t, sep="")
