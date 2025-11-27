n, m = map(int,input().split())

cnt = 0
for i in range(m+1):
    cnt += n ** i

if cnt <= 10 ** 9:
    print(cnt)
else:
    print("inf")
