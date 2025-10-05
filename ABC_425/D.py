from collections import deque

H, W = map(int, input().split())
S = [list(input()) for _ in range(H)]

def move(x, y):
    cnt = 0
    for i, j in dxdy:
        nx, ny = x + i, y + j
        if 0 <= nx < H and 0 <= ny < W and S[nx][ny] == "#":
            cnt += 1

    return cnt

colect = []
for i in range(H):
    for j in range(W):
        if S[i][j] == "#":
            colect.append((i, j))

dxdy = [(1, 0), (-1, 0), (0, 1), (0, -1)]
for i in range(H * W):
    temp = []
    for x, y in colect:
        for dx, dy in dxdy:
            nx, ny = x + dx, y + dy
            if 0 <= nx < H and 0 <= ny < W and move(nx, ny) == 1:        
                temp.append((nx, ny))

    if len(temp) == 0:
        break
    
    colect = []
    for x, y in temp:
        if S[x][y] == "#":
            continue
        S[x][y] = "#"
        colect.append((x, y))

ans = 0
for i in range(H):
    for j in range(W):
        if S[i][j] == "#":
            ans += 1

print(ans)