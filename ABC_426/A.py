x, y = input().split()

d = {"Ocelot": 0, "Serval": 1, "Lynx": 2}

if d[x] >= d[y]:
    print("Yes")

else:
    print("No")
