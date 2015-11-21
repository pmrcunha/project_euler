x = 0
for i in range(1,20):
    while x % i == 0:
        if i == 20:
            print x
        else:
            x += 1

