n, m = map(int,input().split())
A = list(map(int, input().split()))
D = {}
for i in range(n):
    idx = (A[i] % m)
    s = len(str(A[i]))
    tup = (idx, s)
    if tup in D:
        D[tup] += 1
    else:
        D[tup] = 1
ans = 0
for i in range(n):
    for j in range(1, 10+1):
        idx = ((A[i] % m) * pow(10, j, m)) % m
        tup = ((m-idx)%m, j)
        if tup in D:
            ans += D[tup]

print(ans)
