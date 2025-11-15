x = int(input())
n = int(input())
W = list(map(int, input().split()))
q = int(input())

t = [False] * n
for i in range(q):
    p = int(input()) - 1
    if t[p]:
        x -= W[p]
    else:
        x += W[p]
    t[p] = not t[p]
    print(x)
