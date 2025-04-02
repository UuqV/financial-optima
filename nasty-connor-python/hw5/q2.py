from q1 import call, vega

approx = 1e-6

s = k = 40
q = .01
r = .025
t = (5/12)
price = 2.75


def bisection(left, right):
    bs_l = call(left, s, k, t, r, q) - price
    bs_r = call(right, s, k, t, r, q) - price
    while abs(bs_l - bs_r) > approx or abs(left - right) > approx:

        midpoint = (left + right) / 2
        bs_m = call(midpoint, s, k, t, r, q) - price
        if bs_m < 0:
            left = midpoint
        elif bs_m > 0:
            right = midpoint
        bs_l = call(left, s, k, t, r, q)
        bs_r = call(right, s, k, t, r, q)
    # print(left)
    # print(right)

init_l = .0001
init_r = 1
bisection(init_l, init_r)


def newton(init_guess):
    xold = -1000
    xnew = init_guess
    tol_approx = 1e-6
    while abs(xold - xnew) > tol_approx:
        xold = xnew
        sub = (call(xnew, s, k, t, r, q) - price)/vega(xnew, s, k, r, t, q)
        xnew = xnew - sub

    print(xnew)
# newton(.5)


def secant(xold, xnew):
    while abs(call(xnew, s, k, t, r, q)) - price > approx or abs(xold - xnew) > approx:
        xoldest = xold
        xold = xnew
        fxold = call(xold, s, k, t, r, q) - price
        fxoldest = call(xoldest, s, k, t, r, q) - price
        diff = (xold - xoldest) / (fxold - fxoldest)
        xnew = xold - fxold * diff
    print(xnew)

x0 = .6
x1 = .5
secant(x0, x1)

