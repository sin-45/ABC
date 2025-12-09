from collections import deque
h, w = map(int,input().split())
grid = [list(input()) for i in range(h)]

que = deque()
for i in range(h):
    for j in range(w):
        if grid[i][j] == "E":
            que.append([i, j])

xy = [[-1, 0, "v"], [1, 0, "^"], [0, -1, ">"], [0, 1, "<"]]
while que:
    x, y = que.popleft()
    for i, j, c in xy:
        nx, ny = x+i, y+j
        if 0 <= nx < h and 0 <= ny < w:
            if grid[nx][ny] == ".":
                que.append([nx, ny])
                grid[nx][ny] = c

for i in grid:
    print("".join(i))
