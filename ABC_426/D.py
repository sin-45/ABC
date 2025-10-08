t = int(input())

for i in range(t):
    n = int(input())
    s = input()
    zero, one = 0, 0
    c = (n+1) // 2
    rev = n - c
    # print("->", c, rev, n, s)
    # print(s[:c], s[n-rev:n])
    # zero
    zero_temp = 0
    one_temp = 0
    for j in range(c):
        if s[j] == "0":
            zero_temp += 2
            one += 1 + one_temp
            one_temp = 0
        else:
            zero += 1 + zero_temp
            one_temp += 2
            zero_temp = 0

    zero_temp = 0
    one_temp = 0
    for j in range(rev):
        if s[n-j-1] == "0":
            zero_temp += 2
            one += 1 + one_temp
            one_temp = 0
        else:
            zero += 1 + zero_temp
            one_temp += 2
            zero_temp = 0

    print(min(zero, one))
