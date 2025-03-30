import math
import scipy

from q1 import d1


def delta(S, K, r, sig, T, q):
    eq = math.exp(-q * T)
    d_1 = d1(S, K, r, sig, T, q)
    return eq * scipy.stats.norm.cdf(d_1)


def delta_derive_by_k(S, K, r, sig, T, q):
    # I still think it's bullshit k represents strike 
    d_1 = d1(S, K, r, sig, T, q)
    num = -1 * math.exp(-1 * q * T) * scipy.stats.norm.pdf(d_1)
    denom = K * sig * math.sqrt(T)

    return num / denom 


def main(S, xnew, r, sig, T, q):
    xold = -1000
    tol_approx = 1e-6
    while abs(xold - xnew) > tol_approx:
        xold = xnew
        sub = (delta(S, xnew, r, sig, T, q) - .5)/delta_derive_by_k(S, xnew, r, sig, T, q)
        xnew = xnew - sub
        print(xnew)


init_K = 30
sig = .3
q = .01
r = .025
S = 30
t = (3/12)

main(S, init_K, r, sig, t, q)


