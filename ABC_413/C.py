q = int(input())

a = []
idx = 0
for _ in range(q):
    s = list(map(int, input().split()))
    if s[0] == 1:
        a.append([s[1], s[2]])
    else:
        cnt = 0
        while True:
            if s[1] <= a[idx][0]:
                cnt += s[1] * a[idx][1]
                a[idx][0] -= s[1]
                print(cnt)
                break
            else:
                cnt += a[idx][0] * a[idx][1]
                s[1] -= a[idx][0]
                idx += 1

