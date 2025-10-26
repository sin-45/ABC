n, m = map(int,input().split())
A = list(map(int,input().split()))
sum_a = sum(A)

t = False
for i in A:
    if sum_a - i == m:
        t = True

if t:
    print("Yes")
else:
    print("No")
    