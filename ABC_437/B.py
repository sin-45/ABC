h, w, n = map(int,input().split())
A = [list(map(int,input().split())) for _ in range(h)]

F = [0] * h
for i in range(n):
    b = int(input())
    for j in range(h):
        if b in A[j]:
            F[j] += 1

print(max(F))
