n, k = map(int,input().split())
A = [1] * k
cnt = k
mod = 10 ** 9
for i in range(n-k+1):
    cnt %= mod
    A.append(cnt)
    cnt += A[-1] - A[i]
print(A[-1])
