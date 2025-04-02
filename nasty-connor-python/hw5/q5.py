import math
import scipy

from q1 import d1, d2


def theta(S, K, r, sig, T, q):
    d_1 = d1(S, K, r, sig, T, q)
    d_2 = d2(sig, T, d_1)
    strike_term = r * K * math.exp(-r * T) * scipy.stats.norm.cdf(-d_2)
    spot_term = (S * sig) / (2 * math.sqrt(T)) * scipy.stats.norm.pdf(d_1)
    return strike_term - spot_term

def theta_deriv_log_moneyness(S, K, r, sig, T, q):
    d_1 = d1(S, K, r, sig, T, q)
    d_2 = d2(sig, T, d_1)
    m = S/K
    first_1 = r * math.exp(-1 * r * T)
    first_2 = -1 * scipy.stats.norm.pdf(-1 * d_2) / (m * sig * math.sqrt(T))
    first_3 = -1 * S * scipy.stats.norm.cdf(-1 * d_2) / math.pow(m, 2)

    second_1 = -1 * sig / (2 * math.sqrt(2 * math.pi * T))
    second_2 = K * math.exp(-1 * math.pow(d_1, 2) / 2)
    second_3 = S * math.exp(-1 * math.pow(d_1, 2) / 2)  * (-1 * d_1) / (m * sig * math.sqrt(T))

    return first_1 * (first_2 + first_3) + second_1 * (second_2 + second_3)
    

def main(guess_S, K, r, sig, T, q):
    xold = -1000
    xnew = guess_S/K
    tol_approx = 1e-6
    print(guess_S)
    while abs(xold - xnew) > tol_approx:
        xold = xnew
        sub = theta(xnew * 2, 2, r, sig, T, q)/theta_deriv_log_moneyness(xnew * 2, 2, r, sig, T, q)
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
print(theta(73.4724 * 2, 200, r, sig, T, 0))
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
