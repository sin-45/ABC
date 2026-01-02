t = input()
u = input()

ans = False
for i in range(len(t)-len(u)+1):
    for j in range(len(u)):
        if t[i+j] == "?": continue
        if t[i+j] != u[j]:
            break

    else:
        ans = True
        break

if ans:
    print("Yes")
else:
    print("No")