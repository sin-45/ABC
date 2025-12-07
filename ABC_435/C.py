n = int(input())
A = list(map(int,input().split()))

r = A[0]
ans = 1
for i in range(1, n):
    if i < r:
        r = max(r, A[i] + i)
        ans += 1

print(ans)
