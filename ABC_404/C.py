from collections import deque
n, m = map(int,input().split())
S = set()
L = [[] for _ in range(n+1)]
for i in range(m):
    a, b = map(int,input().split())
    L[a].append(b)
    L[b].append(a)
    start = a

if m != n:
    print("No")
    exit()

t = [False] * (n+1)
t[start] = True
que = deque()
que.append(L[start][0])

while que:
    q = que.pop()
    if t[q]:
        print("No")
        exit()
    t[q] = True
    for i in L[q]:
        if t[i]: continue
        que.append(i)

print("Yes")
