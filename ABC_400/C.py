n = int(input())

ans = 0
for i in range(1, 2+1):
    r = 10 ** 10
    l = 0
    while l+1 != r:
        mid = (l + r) // 2
        if 2 ** i * (mid * mid) <= n:
            l = mid
        else:
            r = mid
    ans += l

print(ans)

    