s, a, b, x = map(int, input().split())
cnt = x // (a+b)
ans = s * cnt * a + min(x % (a+b), a) * s
print(ans)