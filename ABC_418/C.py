from bisect import bisect_left, bisect_right
from math import perm

n, q = map(int,input().split())
A = list(map(int,input().split()))
A.sort()
F = [0]
for i in A:
    F.append(F[-1] + i)

for _ in range(q):
    b = int(input())
    if A[-1] < b:
        print(-1)
        continue
    ans = 0
    c = bisect_left(A, b)
    ans += F[c]
    ans += (n-c) * (b-1) + 1
    print(ans)
