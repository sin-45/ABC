t = int(input())

def point(x0, y0, x1, y1):
    if x0 == x1:
        return x0, 1, 0
    else:
        a = (y1-y0) / (x1-x0)
        b = 1
        c = y1

        return a, b, c

def dist(x, y, a, b, c):
    s = pow(a*a + b*b, 1/2)
    f = abs(a*x + b*y + c)
    return f / s

def pert(x, a, b, c):
    return a*x + c

for i in range(t):
    T = list(map(int, input().split()))
    A = list(map(int, input().split()))

    t_list = point(*T)
    a_list = point(*A)

    left = min(A[0], A[2])
    right = min(A[0], A[2])
    # while True:
    for i in range(10 ** 3):
        yl = pert(left, *a_list)
        zl = dist(left, yl, *t_list)
        yr = pert(right, *a_list)
        zr = dist(right, yr, *t_list)

        if zl < zr:
            zr = (left + right) / 2
        else:
            zl = (left + right) / 2
        
    print(zl)
