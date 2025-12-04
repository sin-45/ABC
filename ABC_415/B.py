s= input()

c = -1
for i in range(len(s)):
    if s[i] == "#":
        if c == -1:
            c = i
        else:
            print(f"{c+1},{i+1}")
            c = -1

