n, m = map(int, input().split())
s = [0] * n
S = [input() for _ in range(n)]

for i in range(m):
    t = input()
    cnt = 0
    key = -1
    ok = False
    for j in range(n):
        if t == S[j]:
            ok = True
            s[j] += 1

        if len(t) == len(S[j]):
            temp = 0
            for k in range(len(t)):
                if t[k] != S[j][k]: temp += 1

            if temp <= 1:
                cnt += 1
                key = j

    if (not ok) and cnt == 1:
        s[key] += 1

print(*s, sep="\n")

