n, q = map(int, input().split())
A = list(map(int, input().split()))
B = list(map(int, input().split()))
ans_list = [min(a, b) for a, b in zip(A, B)]
ans_sum = sum(ans_list)

for _ in range(q):
    c, x, y = input().split()
    x, y = int(x), int(y)
    if c == "A":
        ans_sum -= min(A[x-1], B[x-1])
        A[x-1] = y
        ans_sum += min(A[x-1], B[x-1])

    else:
        ans_sum -= min(A[x-1], B[x-1])
        B[x-1] = y
        ans_sum += min(A[x-1], B[x-1])

    print(ans_sum)
    