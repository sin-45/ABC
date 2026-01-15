n, m = map(int,input().split())
s = list(map(int, list(input())))
t = list(map(int, list(input())))

cnt = 10000
for i in range(n-m+1):
    temp = 0
    for j in range(m):
        temp += abs((10 + s[i+j] - t[j]) % 10)

    cnt = min(cnt, temp)

print(cnt)

