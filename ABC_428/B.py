n, k = map(int,input().split())
s = input()

A = dict()
mx = 1
for i in range(n-k+1):
    t = s[i:i+k]
    if t in A:
        A[t] += 1
        mx = max(mx, A[t])

    else:
        A[t] = 1

ans = []
for i in A:
    if A[i] == mx:
        ans.append(i)

print(mx)
ans.sort()
print(*ans)