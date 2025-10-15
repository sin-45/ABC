n = int(input())
A = [1]

def f(t):
    s = sum(list(map(int, "".join(str(t)))))
    return s

for i in range(n):
    A.append(A[-1] + f(A[-1]))

print(A[-2])
