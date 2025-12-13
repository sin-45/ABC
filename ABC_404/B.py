n = int(input())
S = [list(input()) for _ in range(n)]
T = [list(input()) for _ in range(n)]

ans = 10000
for k in range(4):
    c = k
    if k != 0:
        S1 = [["" for j in range(n)] for i in range(n)]
        for i in range(n):
            for j in range(n):
                S1[j][n-i-1] = S[i][j]

        S = S1
    
    for i in range(n):
        for j in range(n):
            if S[i][j] != T[i][j]:
                c += 1

    ans = min(ans, c)

print(ans)

    