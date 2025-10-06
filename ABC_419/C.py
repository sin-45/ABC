n = int(input())
r, c = [], []

for i in range(n):
    a, b = map(int, input().split())
    r.append(a)
    c.append(b)

x = max(r) - min(r)
y = max(c) - min(c)
ans = (max(x, y) + 1) // 2
print(ans)
