n = int(input())
T = list(map(int, input().split()))
F = [[i, T[i]] for i in range(n)]
F.sort(key=lambda x: x[1])
for i in range(3):
    print(F[i][0] + 1, end=' ')
print()