n, m = map(int,input().split())
rev_tag = [[] for _ in range(n+1)]
for i in range(m):
    x, y = map(int,input().split())
    rev_tag[y].append(x)

kuro = [False] * (n+1)
q = int(input())
no_set = set()
for _ in range(q):
    a, v = map(int,input().split())
    if a == 1:
        stack = [v]
        while stack:
            q = stack.pop()
            if kuro[q]:
                continue
            kuro[q] = True
            for i in rev_tag[q]:
                if not kuro[i]:
                    stack.append(i)
    else:
        if kuro[v]:
            print("Yes")
        else:
            print("No")
