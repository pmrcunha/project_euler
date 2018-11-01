#01 Add all the multiples of 3 and 5 up to 1000

x = 0
for c in range(1000):
    if c % 5 == 0 or c % 3 == 0:
        x += c

print x
