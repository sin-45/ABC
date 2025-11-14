n, a, b = map(int,input().split())
s = input()
l = 0
r = 0

a_cnt = 0
b_cnt = 0
cnt = 0
while r != n:
    mark = "l"
    if b_cnt < b:
        if a_cnt >= a:
            cnt += (a_cnt - a + 1)
        mark = "r"

    if mark == "l":
        if s[l] == "a":
            a_cnt -= 1
        else:
            b_cnt -= 1
        l += 1
    else:
        if s[r] == "a":
            a_cnt += 1
        else:
            b_cnt += 1
        r += 1
        
print(cnt)