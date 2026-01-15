d, f = map(int,input().split())
if f - d % 7 == 0:
    print(7)
else:
    print((f - d) % 7)
