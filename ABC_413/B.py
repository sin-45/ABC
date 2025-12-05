n = int(input())
D = set()
S = []
for _ in range(n):
    s = input()
    S.append(s)

for i in range(n):
    for j in range(n):
        if i == j:
            continue
        D.add(S[i] + S[j])

print(len(D))
