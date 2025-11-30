n, m = map(int,input().split())

A = [0] * 100
B = [0] * 100
for i in range(n):
    a, b = map(int,input().split())
    A[a-1] += b
    B[a-1] += 1

for i in range(100):
    if B[i] != 0:
        print(A[i] / B[i])
    