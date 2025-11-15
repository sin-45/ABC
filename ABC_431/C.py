n, m, k = map(int,input().split())
H = list(map(int,input().split()))
B = list(map(int,input().split()))

H.sort()
B.sort()

h = 0
cnt = 0
for i in range(m):
    if H[h] <= B[i]:
        cnt += 1
        h += 1
    
    if h == n:
        break

if cnt >= k:
    print("Yes")
else:
    print("No")
