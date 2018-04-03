import sys

alphabets = "abcdefghijklmnopqrstuvwxyz"
S = input()
mp = {}

for s in S:
    if s in mp.keys():
        mp[s] = mp[s] + 1
    else:
        mp[s] = 1

def idx(c):
    if len(c) != 1:
        print("error")
        return None # invalid
    for i, e in enumerate(alphabets):
        if c == e:
            return i
    print("error")
    return None # invalid

# 使われてない文字がある場合
unused = []
for a in alphabets:
    if a not in mp.keys():
        unused.append(a)
        
if len(unused) > 0:
    print(S + unused[0])
    sys.exit(0)

# 先頭から探していく
pos = len(S) - 1
while True:
    if pos == 0:
        print(-1)
        sys.exit()
    x = S[pos-1]
    y = S[pos]
    if idx(x) < idx(y):
        # 書き換え
        minc = 'z'
        unused.append(y)
        for c in unused:
            if idx(minc) > idx(c) and idx(c) > idx(x):
                minc = c
        print(S[:pos-1] + minc)
        sys.exit()
    else:
        unused.append(y)

    # update
    pos = pos - 1

"""
abcdefghijklmnopqrstuvwzyx
abcdefghijklmnopqrstuvzyxw
azyxwvutsrqponmlkjihgfedcb
"""