s = input()

m = ""
c = 0
for i in range(len(s)):
    if m != s[i]:
        m = s[i]
        c += 1

A = []
B = []
m = ""
for i in range(len(s)):
    if m != s[i]:
        A.append(int(s[i]))
        B.append(1)
        m = s[i]
    else:
        B[-1] += 1

ans = 0
for i in range(len(A)-1):
    if A[i] + 1 == A[i+1]:
        ans += min(B[i], B[i+1])

print(ans)
