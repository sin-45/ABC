n, m = map(int,input().split())
S = [list(input()) for _ in range(n)]

cnt = set()
for i in range(n-m+1):
    for j in range(n-m+1):
        temp = []
        for k in range(m):
            for l in range(m):
                temp.append(S[i+k][j+l])
        
        cnt.add(tuple(temp))

print(len(cnt))
