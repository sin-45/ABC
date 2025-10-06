n, m = map(int,input().split())
s = input()
t = input()
y = [s, t]

ans = [0] * (n+1)
for i in range(m):
    a, b = map(int,input().split())
    ans[a-1] += 1
    ans[b] -= 1

c = 0
x = []
for i in range(n):
    c += ans[i]
    x.append(y[c%2][i])

print("".join(x))