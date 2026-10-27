q = int(input())

d = dict()
cnt = 1
for i in range(q):
    s = input()
    if s in d:
        print(d[s])
    else:
        print(cnt)
        d[s] = cnt
        cnt += 1
