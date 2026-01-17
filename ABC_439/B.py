n = list(map(int, list(input())))
h = 0

for i in range(10 ** 6):
    h = sum([int(i)*int(i) for i in n])
    if h == 1:
        print("Yes")
        exit()
    n = list(str(h))

print("No")
