n, m = map(int,input().split())
eg = [list(map(int,input().split())) for _ in range(m)]

ans = m
for i in range(1 << n):
    cnt = 0
    for u, v in eg:
        if (1 & (i >> u)) == (1 & (i >> v)):
            cnt += 1

    ans = min(ans, cnt)

print(ans)