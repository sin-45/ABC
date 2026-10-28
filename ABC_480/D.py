from bisect import bisect_left, bisect_right

n, q = map(int, input().split())
A = list(map(int, input().split()))

s = [0] * (n+1)
ss = [0] * (n+1)
c = [0] * (n+1)
for i in range(n):
    s[i+1] = A[i] + s[i]
    ss[i+1] = s[i+1] + ss[i]
    c[i+1] = i+1 + c[i]


for i in range(q):
    l, r = map(int, input().split())
    l -= 1
    lc = bisect_right(c, l) - 1
    rc = bisect_right(c, r) - 1
    ans = ss[rc] + s[r-c[rc]] - ss[lc] - s[l-c[lc]]
    print(ans)

