n = int(input())
cnt = 0
for i in range(1, n+1):
    cnt += pow(-1, i) * pow(i, 3)

print(cnt)
