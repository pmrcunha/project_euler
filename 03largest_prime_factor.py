def getPrimeFactors(n):
    return listFactors([n])

def listFactors(input_list):
    l = input_list
    p = 2
    while l[0] % p != 0:
        p += 1
    else:
        if l[0] == p:
            return l
        else:
            l[0] /= p
            l.append(p)
            return listFactors(l)

print max(getPrimeFactors(600851475143))
        
    
