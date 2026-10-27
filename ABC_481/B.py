n, k = map(int, input().split())
a = list(map(int, input().split()))

cnt = a[0] == k
for i in range(n-1):
    cnt += a[i+1] == k
    if min(a[i], a[i+1]) < k < max(a[i], a[i+1]):
        cnt += 1
        
print(cnt)
