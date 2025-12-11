s = input()
ans = 0
for i in range(len(s)-2):
    for j in range(i+2, len(s)):
        cnt = s[i:j+1].count("t")
        ans = max(ans, (cnt-2) / (j-i-1))

print(ans)
