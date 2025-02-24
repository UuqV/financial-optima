import time 
import math 

def calculate(left, right, n):
    def normal_var(x):
        exp = math.pow(1 + x, 2) * -1
        val = .05/(1 + (2*exp))
        # print(f"{x}: {math.exp(-(math.pow(x,2)/2))} ")
        return val 
    
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
    retVal *= -1

    print(f"For {n}: {retVal}")
    return retVal 

def driver(left, right, n, tol=1e-12):
    prevVal = -1000000

    retVal = calculate(left, right, n)
    while abs(retVal - prevVal) > tol:
        n *= 2
        prevVal = retVal
        retVal = calculate(left, right, n)
        # time.sleep(2)
    
    return retVal 

 
if __name__ == "__main__":
    print("6 months")
    six_month = driver(left=0, right=.5, n=4, tol=1e-6) 
    print(six_month)

    print("1 year")
    one_year = driver(left=0, right=1, n=4, tol=1e-6)
    print(one_year)

    print("18 months")
    eighteen_month = driver(left=0, right=1.5, n=4, tol=1e-6)
    print(eighteen_month)

    print("2 years")
    two_year = driver(left=0, right=2, n=4, tol=1e-8)
    print(two_year)

    bond_price = 0
    # Coupon is 2.5, so 2.5, 2.5, 2.5, 102.5 
    
    first = 2.5 * math.exp(-(1/2) * six_month)
    second = 2.5 * math.exp(-1 * one_year)
    third = 2.5 * math.exp(-(3/2) * eighteen_month)
    fourth = 102.5 * math.exp(-2 * two_year)
    print(f"Disc six month: {first}")
    print(f"Disc one year: {second}")
    print(f"Disc eighteen month: {third}")
    print(f"Disc two year: {fourth}")
    print(f"Price 2 year {first + second + third + fourth}")


