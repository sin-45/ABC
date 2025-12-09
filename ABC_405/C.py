n = int(input())
A = list(map(int,input().split()))

B = [0]
for i in A:
    B.append(B[-1] + i)

ans = 0
mx = B[-1]
for i in range(n-1):
    ans += A[i] * (mx-B[i+1])

print(ans)
