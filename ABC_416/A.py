n, l, r = map(int, input().split())
s = input()

t = "o" * (r-l+1)
if s[l-1:r] == t:
    print("Yes")
else:
    print("No")
