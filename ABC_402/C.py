from collections import defaultdict
n, m = map(int,input().split())
K = [list(map(int,input().split()))[1:] for _ in range(m)]
B = list(map(int,input().split()))

D = defaultdict(set)
for i in range(m):
    for k in K[i]:
        D[k].add(i)

cnt = [len(set(i)) for i in K]

ans = 0
for i in range(n):
    for k in D[B[i]]:
        cnt[k] -= 1
        if cnt[k] == 0:
            ans += 1
    print(ans)
