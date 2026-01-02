from collections import defaultdict
n, m, q = map(int, input().split())
D = defaultdict(set)
T = [False] * n

for _ in range(q):
    inp = list(map(int, input().split()))
    t = inp[0]
    if t == 1:
        D[inp[1]-1].add(inp[2]-1)
    elif t == 2:
        T[inp[1]-1] = True
    else:
        a = inp[1]-1
        b = inp[2]-1
        if b in D[a] or T[a]:
            print("Yes")
        else:
            print("No")
