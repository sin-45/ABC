from itertools import permutations, combinations_with_replacement, product
n, k, x = map(int,input().split())

S = []
for i in range(n):
    s = input()
    S.append(s)

D = []
for i in product(S, repeat=k):
    D.append("".join(i))
    
D.sort()
print(D[x-1])