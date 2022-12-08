def priority(s):
    if ord(s) >= ord('a') and ord(s) <= ord('z'):
        return ord(s) - ord('a') + 1
    else:
        return ord(s) - ord('A') + 1

with open('input') as f:
    total = 0
    for row in f:
        mid = int(len(row) / 2)
        c =  set(row[:mid]) & set(row[mid:])
        total += priority(list(c)[0])
    print(total)
