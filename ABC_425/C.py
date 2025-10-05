n, q = map(int, input().split())
A = list(map(int, input().split()))

now = 0
B = A * 2
C = [0]
for i in range(2*n):
    C.append(C[-1] + B[i])

for i in range(q):
    X = list(input().split())
    if X[0] == "1":
        now += int(X[1])
        now %= n

    else:
        l, r = int(X[1]), int(X[2])
        print(C[r+now] - C[l-1+now])
