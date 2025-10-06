q = int(input())

temp = []
for i in range(q):
    a = list(input().split())
    if a[0] == "1":
        temp.append(int(a[1]))

    elif a[0] == "2":
        print(min(temp))
        temp.remove(min(temp))


