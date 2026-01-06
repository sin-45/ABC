n, m = map(int,input().split())
S = set()

for _ in range(m):
    r, c = map(int,input().split())
    r, c = r-1, c-1
    t = False
    for i in range(2):
        for j in range(2):
            if (r+i, c+j) in S:
                t = True

    if t:
        continue
    for i in range(2):
        for j in range(2):
            S.add((r+i, c+j))

print(len(S) // 4)
