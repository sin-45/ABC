from bisect import bisect_right
t = int(input())
for _ in range(t):
    n = int(input())
    A = []
    p_cnt = 0
    for i in range(n):
        w, p = map(int, input().split())
        p_cnt += p
        A.append(w+p)
    A.sort()
    for i in range(n):
        p_cnt -= A[i]
        if p_cnt < 0:
            print(i)
            break


