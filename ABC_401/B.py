n = int(input())
t = False
cnt = 0
for i in range(n):
    s = input()
    if s == "login":
        t = True
    elif s == "logout":
        t = False
    elif s == "private":
        if not t:
            cnt += 1

print(cnt)