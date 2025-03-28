import math
import scipy

from q1 import d1


def delta(S, K, r, sig, T, q):
    eq = math.exp(-q * T)
    d_1 = d1(S, K, r, sig, T, q)
    return eq * scipy.stats.norm.cdf(d_1)


def gamma(S, K, r, sig, T, q):
    d_1 = d1(S, K, r, sig, T, q)
    first_num = math.exp(-q * T)
    first_denom = S * sig * math.sqrt(T)

    return (first_num / first_denom) * scipy.stats.norm.pdf(d_1)


def main(xnew, K, r, sig, T, q):
    xold = -1000
    tol_approx = 1e-6
    while abs(xold - xnew) > tol_approx:
        xold = xnew
        sub = (delta(xnew, K, r, sig, T, q) - .5)/gamma(xnew, K, r, sig, T, q)
        xnew = xnew - sub
        print(xnew)


K = 30
sig = .3
q = .01
r = .025
init_S = 30
t = (3/12)

main(init_S, K, r, sig, t, q)


