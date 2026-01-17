from bisect import bisect_left, bisect_right
n, q = map(int,input().split())
A = list(map(int,input().split()))
S = set()
F = []
t = False
A.sort()
for i in range(n-1):
    if A[i] + 1 == A[i+1]:
        S.add(A[i])
        t = True
        continue
    if t:
        F.append(A[i])
        t = False
if t:
    F.append(A[i])
    t = False

for _ in range(q):
    x, y = map(int,input().split())
    l = 0
    r = 10 ** 10
    while l+1 != r:
        c = (l+r)//2
        if bisect_right(A, c) - bisect_left(A, x) + y + x < c:
            r = c
        else:
            l = c
    print(l-1) 

