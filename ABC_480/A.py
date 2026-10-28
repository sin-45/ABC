s1 = input()
s2 = input()

a, b = s1[0], s1[1]
c, d = s2[0], s2[1]

f = False
if a == "A" and (b == "D" or c == "D"): f = True
if b == "A" and (a == "D" or d == "D"): f = True
if c == "A" and (a == "D" or d == "D"): f = True
if d == "A" and (b == "D" or c == "D"): f = True

if f:
    print("Yes")
else:
    print("No")
    