n = int(input())
A = list(map(int,input().split()))

cnt = 0
F = [-1] * n
S = set(list(range(1, n+1)))

for i in range(n):
    if A[i] == -1:
        cnt += 1

    else:
        if A[i] in S:
            S.remove(A[i])
            F[i] = A[i]
        else:
            print("No")
            exit()
            
print("Yes")
for i in range(n):
    if F[i] == -1:
        a = S.pop()
        F[i] = a

print(*F)