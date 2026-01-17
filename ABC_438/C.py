n = int(input())
a = list(map(int,input().split()))

stack = [[0, 0]]
for i in a:
    if stack[-1][0] == i:
        stack[-1][1] += 1
        if stack[-1][1] == 4:
            stack.pop()
    else:
        key = i
        cnt = 1
        stack.append([i, cnt])

print(sum([j for i, j in stack]))