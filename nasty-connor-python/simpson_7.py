import numpy
import time 
import math 

left = 0
right = 1
n = 4 
h = (right-left)/n
tol = 1e-12


def calculate(left, right, n):
    def normal_var(x):
        # print(f"{x}: {math.exp(-(math.pow(x,2)/2))} ")
        return math.exp(-(math.pow(x,2)/2))
    
    h = (right-left) / n
    retVal = normal_var(left)/6 + normal_var(right)/6
    # print(f"For first two {retVal}")

    for i in range(1, n):
        retVal += normal_var(left + i * h) / 3 
        # print(f"i {i} for interval {left + i * h} : {retVal}")

    for l in range(1, n + 1):
        retVal += 2 * (normal_var(left + (l - .5) * h)/3)
        # print(f"l {l} for interval {left + (l - .5) * h}: {retVal}")

    retVal *= h

    retVal *= (1 / math.sqrt(math.pi * 2))
    retVal += .5
    print(f"For {n}: {retVal}")
    return retVal 

def driver(left, right, n, tol=1e-12):
    prevVal = -1000000

    retVal = calculate(left, right, n)
    while abs(retVal - prevVal) > tol:
        n *= 2
        prevVal = retVal
        retVal = calculate(left, right, n)
        # time.sleep(3)
    
    return retVal 



 
if __name__ == "__main__":
    x = driver(left=0, right=.1, n=4) 
    print(x)
    x = driver(left=0, right=.5, n=4) 
    print(x)
    x = driver(left=0, right=1, n=4) 
    
    print(x)
    
