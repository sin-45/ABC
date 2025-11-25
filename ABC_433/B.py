n = int(input())
A = list(map(int,input().split()))

for i in range(n):
    c = -1
    for j in range(i):
        if A[j] > A[i]:
            c = j+1

    print(c)
