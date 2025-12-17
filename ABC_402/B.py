from collections import deque
q = int(input())

que = deque()
for _ in range(q):
    a = list(map(int,input().split()))
    if a[0] == 1:
        x = a[1]
        que.append(x)
    else:
        p = que.popleft()
        print(p)
