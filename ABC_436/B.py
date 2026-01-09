n = int(input())

grid = [[0] * n for _ in range(n)]
grid[0][(n-1)//2] = 1
r, c = 0, (n-1) // 2
k = 1
for i in range(n*n-1):
    if grid[(r-1) % n][(c+1) % n] == 0:
        grid[(r-1) % n][(c+1) % n] = k+1
        r, c = (r-1) % n, (c+1) % n
    else:
        grid[(r+1) % n][c] = k+1
        r = (r+1) % n
    k += 1
    
for i in grid:
    print(*i)
