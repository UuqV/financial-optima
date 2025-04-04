import math
import scipy


def d1(S, K, r, sig, T, q):
    num = math.log(S/K) + (r - q + (math.pow(sig, 2)/2)) * T
    denom = sig * math.sqrt(T)
    return num / denom


def d2(sig, T, d_1):
    return d_1 - (sig * math.sqrt(T))


def call(sig, S, K, T, r, q):
    d_1 = d1(S, K, r, sig, T, q)
    d_2 = d2(sig, T, d_1)
    val = S * math.exp(-q * T) * scipy.stats.norm.cdf(d_1) - K * math.exp(-r * T) * scipy.stats.norm.cdf(d_2)
    return val

def vega(sig, S, K, r, T, q):
    d_1 = d1(S, K, r, sig, T, q)
    ed_1 = scipy.stats.norm.pdf(d_1)
    v = S * math.exp(-q * T) * ed_1 * math.sqrt(T)
    return v


def main():
    k = 30 
    spot = 30 
    time = .5 
    interest = .03 
    divs = .01
    xold = -1000
    xnew = .5
    tol_approx = 1e-6
    while abs(xold - xnew) > tol_approx:
        xold = xnew
        sub = (call(xnew, spot, k, time, interest, divs) - 2.5)/vega(xnew, spot, k, interest, time, divs)
        xnew = xnew - sub
        print(xnew)


if __name__ == "__main__":
    main()
