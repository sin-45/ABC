from collections import deque
import sys

h, w = map(int, input().split())
grid = [input() for _ in range(h)]

dist = [[[-1] * w for _ in range(h)] for k in range(2)]
for i in range(h):
    for j in range(w):
        if grid[i][j] == "S":
            dist[0][i][j] = 0
            q = deque([(i, j, 0, 0)])

dxdy = [(1, 0), (0, 1), (-1, 0), (0, -1)]
while q:
    x, y, d, t = q.popleft()
    for dx, dy in dxdy:
        nx, ny = x + dx, y + dy
        if 0 <= nx < h and 0 <= ny < w:
            if grid[nx][ny] == "G":
                print(d + 1)
                sys.exit()

            elif (grid[nx][ny] == "." or grid[nx][ny] == "S") and dist[t][nx][ny] == -1:
                dist[t][nx][ny] = d+1
                q.append((nx, ny, d+1, t))

            elif grid[nx][ny] == "?":
                tt = (t+1) % 2
                if dist[tt][nx][ny] == -1:
                    dist[tt][nx][ny] = d+1
                    q.append((nx, ny, d+1, tt))

            elif grid[nx][ny] == "o" and t == 0 and dist[t][nx][ny] == -1:
                dist[t][nx][ny] = d+1
                q.append((nx, ny, d+1, t))

            elif grid[nx][ny] == "x" and t == 1 and dist[t][nx][ny] == -1:
                dist[t][nx][ny] = d+1
                q.append((nx, ny, d+1, t))

print(-1)
