n = int(input())
A = [i*i for i in range(int(pow(10 ** 7, 1/2))+2)]
temp = [0] * (n+1)

for i in range(1, len(A)):
    for j in range(i+1, len(A)):
        if A[i] + A[j] <= n:
            temp[A[i]+A[j]] += 1

ans = []            
for i in range(1, n+1):
    if temp[i] == 1:
        ans.append(i)

print(len(ans))
print(*ans)
