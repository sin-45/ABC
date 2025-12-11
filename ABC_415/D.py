n, m = map(int,input().split())
X = []
for i in range(m):
    a, b = map(int,input().split())
    X.append([a-b, a, b])

X.sort(key=lambda x: x[0])

ans = 0
for k, a, b in X:
    if n < a: continue
    ans += (n-a) // k + 1
    n -= ((n-a) // k + 1) * k

print(ans)
