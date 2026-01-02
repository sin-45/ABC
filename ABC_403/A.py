n = int(input())
A = list(map(int, input().split()))

cnt = sum(A[i] for i in range(n) if i % 2 == 0)
print(cnt)
