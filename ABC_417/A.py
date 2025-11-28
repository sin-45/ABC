n, a, b = map(int,input().split())
s = input()

a, b = min(a, b), max(a, b)
print(s[a:b+1])
