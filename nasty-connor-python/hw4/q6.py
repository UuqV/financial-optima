import math

def txtbook_routine(t: int):
    z = abs(t)
    y = 1/(1+ (0.2316419 * z))
    a1 = .319381530
    a2 = -0.356563782 
    a3 = 1.781477937
    a4 = -1.821255978
    a5 = 1.330274429

    a1y = a1 * y 
    a2y = a2 * math.pow(y, 2)
    a3y = a3 * math.pow(y, 3)
    a4y = a4 * math.pow(y, 4)
    a5y = a5 * math.pow(y, 5)

    ay = a1y + a2y + a3y + a4y + a5y
    exp = math.exp(-(math.pow(t, 2)/2))
    m = 1 - (exp * ay / math.sqrt(2 * math.pi))

    if t > 0:
        nn = m 
    else:
        nn = 1 - m 
    return nn 

def simpson_routine(left, right, n):


    def normal_var(x):
        return math.exp(- (math.pow(x, 2) / 2) )
    
    h = (right-left) / n
    retVal = normal_var(left) / 6 + normal_var(right) / 6

    for i in range(1, n):
        retVal += normal_var(left + i * h) / 3 

    for l in range(1, n + 1):
        retVal += 2 * (normal_var(left + (l - .5) * h)/3)

    retVal *= h

    retVal *= (1 / math.sqrt(math.pi * 2))
    retVal += .5
    print(f"For {n}: {retVal}")
    return retVal 

def driver(left, right, n, tol=1e-12):
    prevVal = -1000000

    retVal = simpson_routine(left, right, n)
    while abs(retVal - prevVal) > tol:
        n *= 2
        prevVal = retVal
        retVal = simpson_routine(left, right, n)
        # time.sleep(3)
    return retVal


def d1(S, K, r, q, sig,T, t):
    num = math.log(S/K) + (r - q + (math.pow(sig,2)/ 2)) * (T-t)
    denom = sig * math.sqrt(T - t) 
    return num / denom 

def d2(sig, T, t, d1):
    return d1 - (sig * math.sqrt(T-t))

def call(t, S, K, T, sig, r, q):
    d_1 = d1(S, K, r, q, sig,T, t)
    print(f"C: d_1 is: {d_1}")
    d_2 = d2(sig, T, t, d_1)
    print(f"C: d_2 is: {d_2}")
    return S * math.exp(-q * (T-t)) * txtbook_routine(d_1) - K * math.exp(-r * (T-t)) * txtbook_routine(d_2)

# def put(t, S, K, T, sig, r, q):
    # d_1 = d1(S, K, r, q, sig,T, t)
    # print(f"P: d_1 is: {d_1}")
    # d_2 = d2(sig, T, t, d_1)
    # print(f"P: d_2 is: {d_2}")
    # return K * math.exp(-r*(T-t)) * txtbook_routine(-1 * d_2) - S * math.exp(-q*(T-t)) * txtbook_routine(-1 * d_1)

def call_simpson(t, S, K, T, sig, r, q):
    d_1 = d1(S, K, r, q, sig,T, t)
    print(f"C: d_1 is: {d_1}")
    d_2 = d2(sig, T, t, d_1)
    print(f"C: d_2 is: {d_2}")
    return S * math.exp(-q * (T-t)) * driver(0, d_1,4,1e-12) - K * math.exp(-r * (T-t)) * driver(0,d_2,4,1e-12)

# Time 
t = 0
# Spot  
S = 40
# Strike 
K = 40
# Maturity Date
T = 0.25
# Volatility 
sig = .2
# Interest Rate
r = .05
# Dividend 
q = .01

print(f"Textbook: {txtbook_routine(0.5)} vs. driver {driver(left=0,right=.5,n=4,tol=1e-12)}")

print(call(t, S, K, T, sig, r, q))
print(call_simpson(t, S, K, T, sig, r, q))
