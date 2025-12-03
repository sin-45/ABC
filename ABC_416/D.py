t = int(input())
for _ in range(t):
    n, m = map(int, input().split())
    A = list(map(int, input().split()))
    B = list(map(int, input().split()))
    A.sort(reverse=True)
    B.sort()
    l = 0
    r = 0
    cnt = 0
    while r != n:
        if A[l] + B[r] >= m:
            cnt += (A[l] + B[r]) % m
            l += 1
            r += 1
        else:
            cnt += B[r]
            r += 1

    for k in range(l, n):
        cnt += A[k]

    print(cnt)
