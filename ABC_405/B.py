n, m = map(int,input().split())
A = list(map(int,input().split()))

ans = n
for i in range(n):
    S = set(A[:n-i])
    t = False
    for j in range(1, m+1):
        if j not in S:
            t = True

    if t:
        ans = i
        break
print(ans)
