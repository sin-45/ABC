import heapq

n, m, s, t = map(int, input().split())
path = [[] for i in range(n+1)]

cnt = 0
for i in range(m):
    a, b = map(int, input().split())
    path[a].append(b)
    if b == t:
        cnt += 1

for i in range(n):
    path[i] = sorted(path[i])

if len(path[s]) == 0 or cnt == 0:
    print(-1)
    exit()


que = []
ok = set()
heapq.heappush(que, (0, s))

while que:
    c, p = heapq.heappop(que)
    if p in ok: continue
    ok.add(p)
    if p == t:
        print(c)
        exit()
    
    for i in range(len(path[p])):
        if path[p][i] in ok: continue
        if i == 0:
            heapq.heappush(que, (c, path[p][i]))
        else:
            heapq.heappush(que, (c+1, path[p][i]))
print(-1)