q = int(input())
F = [0]
G = [0]
for i in range(q):
    A = list(input().split())
    if A[0] == "1":
        val = 1 if A[1] == "(" else -1
        F.append(F[-1] + val)
        G.append(min(F[-1], G[-1]))
    else:
        F.pop()
        G.pop()
    
    if F[-1] == G[-1] == 0:
        print("Yes")
    else:
        print("No")
