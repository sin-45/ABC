from collections import defaultdict

n = int(input())
A = list(map(int,input().split()))

d = defaultdict(int)
for i in A:
    d[i] += 1

cnt = 0
for i in d:
    if d[i] == 1:
        continue
    
    k = n - d[i]
    cnt += k * (d[i]-1) * d[i] // 2

print(cnt)
