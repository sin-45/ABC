n, m = map(int, input().split())
S = [input() for _ in range(n)]
score = [0] * n

for i in range(m):
    zero, one = 0, 0
    for j in range(n):
        if S[j][i] == "0":
            zero += 1
        else:
            one += 1

    if zero == one:
        continue
    elif zero > one:
        for j in range(n):
            if S[j][i] == "1":
                score[j] += 1
    else:
        for j in range(n):
            if S[j][i] == "0":
                score[j] += 1
    
x = max(score)
for i in range(n):
    if score[i] == x:
        print(i + 1, end=" ")
