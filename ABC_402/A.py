s = input()
out = ""
for i in s:
    if ord("A") <= ord(i) <= ord("Z"):
        out += i

print(out)
