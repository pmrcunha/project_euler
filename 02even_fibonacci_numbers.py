fibonacci = [1]
fibonacci_even = []
i = 0
next_value = 2
while next_value < 4000000:
    fibonacci.append(next_value)
    i += 1
    next_value = fibonacci[i - 1] + fibonacci[i]

for n in fibonacci:
    if n % 2 == 0:
        fibonacci_even.append(n)

print sum(fibonacci_even)
