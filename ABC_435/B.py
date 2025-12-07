n = int(input())
A = list(map(int,input().split()))

ans = 0
B = [0]
for i in range(n):
    B.append(B[-1] + A[i])

for i in range(n):
    for j in range(i+1, n+1): 
        k = B[j] - B[i]
        for p in range(i, j):
            if k % A[p] == 0:
                break
        else:
            ans += 1
print(ans)
