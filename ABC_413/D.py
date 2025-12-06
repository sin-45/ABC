t = int(input())
for _ in range(t):
    n = int(input())
    A = list(map(int, input().split()))
    if len(set([abs(x) for x in A])) == 1:
        cnt = 0
        for x in A:
            if x > 0:
                cnt += 1
            else:
                cnt -= 1

        if abs(cnt) <= 1 or abs(cnt) == n:
            print("Yes")
        else:
            print("No")
        continue
    A.sort(key=lambda x: abs(x), reverse=True)
    ok = True
    for i in range(n-2):
        if A[i] * A[i+2] != A[i+1] * A[i+1]:
            ok = False
            break

    if ok:
        print("Yes")
    else:
        print("No")
        