n, k , t = map(int, input().split())
A = list(map(int, input().split()))
A.append(t)
A.append(t+1)
A.sort()
cnt = 0
now = 0
for i in range(n+1):
    if A[i] == A[i+1]: continue
    cnt += (A[i] + k - 1 - now) // k
    now = A[i] + k
    if now >= t:
        break

print(cnt)
