n = int(input())
A = list(map(int,input().split()))

D = {}
ans = 0
for i in range(n):
    if i - A[i] in D:
        ans += D[i-A[i]]
    if i+A[i] in D:
        D[i+A[i]] += 1
    else:
        D[i+A[i]] = 1
print(ans)

