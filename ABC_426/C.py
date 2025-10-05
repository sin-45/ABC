from collections import defaultdict
n, q = map(int,input().split())
A = defaultdict(int)
for i in range(1, n+1):
    A[i] = 1

l = 1
for i in range(q):
    cnt = 0
    x, y = map(int,input().split())
    for j in range(l, x+1):
        cnt += A[j]
        A[j] = 0

    l = max(l, x)
    A[y] += cnt
    print(cnt)
