x, y, z = map(int,input().split())

t = False
for i in range(10 ** 6):
    if (x+i) == (y+i) * z:
        t = True

if t:
    print("Yes")
else:
    print("No")
