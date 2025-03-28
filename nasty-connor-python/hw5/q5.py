import math
import scipy
import time

from q1 import d1, d2


def theta(S, K, r, sig, T, q):
    d_1 = d1(S, K, r, sig, T, q)
    d_2 = d2(sig, T, d_1)
    strike_term = r * K * math.exp(-r * T) * scipy.stats.norm.cdf(-d_2)
    spot_term = (S * sig) / (2 * math.sqrt(T)) * scipy.stats.norm.pdf(d_1)
    return strike_term - spot_term


def theta_deriv_spot(S, K, r, sig, T, q):
    d_1 = d1(S, K, r, sig, T, q)
    d_2 = d2(sig, T, d_1)
    first = -r * K * math.exp(-r * T) * scipy.stats.norm.pdf(-d_2) / (S * sig * math.sqrt(T))
    second_first = sig * math.exp(-math.pow(d_2, 2)/2) / (2 * math.sqrt(2 * T * math.pi))
    second_second = 1 - (d_1/(sig * math.sqrt(T)))
    return first - (second_first * second_second)


def main(guess_S, K, r, sig, T, q):
    xold = -1000
    xnew = guess_S
    tol_approx = 1e-6
    print(guess_S)
    while abs(xold - xnew) > tol_approx:
        xold = xnew
        sub = theta(xnew, K, r, sig, T, q)/theta_deriv_spot(xnew, K, r, sig, T, q)
        xnew = xnew - sub
        print(xnew)
        # time.sleep(1)

    print(xold)
    print(xnew)

#inputs
r = .03
T = .5
sig = .3
q = 0
K = 1
init_s = .7
# main(init_s, K, r, sig, T, 0)
# print(theta(0.7347243353180012, K, r, sig, T, 0))
# print(theta(.1, K, r, sig, T, 0))
# print(theta(.4, K, r, sig, T, 0))
# print(theta(.45, K, r, sig, T, 0))
# print(theta(.5, K, r, sig, T, 0))
# print(theta(.55, K, r, sig, T, 0))
# print(theta(.6, K, r, sig, T, 0))
# print(theta(.65, K, r, sig, T, 0))
# print(theta(.7, K, r, sig, T, 0))  # <-- Here
# print(theta(.75, K, r, sig, T, 0))
# print(theta(.8, K, r, sig, T, 0))
print(theta(1, K, r, sig, T, 0))
# print(theta(.90, K, r, sig, T, 0))
# print(theta(.1, K, r, sig, T, 0))
# So now we know it's a decreasing function, so find theta = 0 as a function of S
#
# print(theta(0.7347243, K, r, sig, T, 0))  # <-- Here
# print(theta(1000, K, r, sig, T, 0))  # <-- Here

# Find max of S/K
# Put  no divs
# r * K * e^{-rt} * N(-d2)
# - (S * sig)/ (2 * sqrt(T)) * pdf(d1)

# So we want S such that theta > 0
# Theta with respect to strike?

# So maximize S and mimimize K, bublackt K contributes to positive side
# Theta should be positive

# Delta P is -N(-d1)
# d1 = (ln(s/k) + t(r + sig^2/2))/(sig * sqrt(t))
