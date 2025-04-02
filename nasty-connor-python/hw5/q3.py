import math


def calc_bond(y_rate, cpn_freq, years, c_rate, face=100):
    result = 0
    cpn_payout = face * c_rate / cpn_freq

    # print(cpn_payout)
    for i in range(cpn_freq * years):
        time = (i + 1) / cpn_freq

        if i == (cpn_freq * years) - 1:
            val = (cpn_payout + face) * math.exp(y_rate * time * -1)
        else:
            val = cpn_payout * math.exp(y_rate * time * -1)
        # print(f"Cash flow is {val} for time {time} and accumulative is {val + result}")
        result += val

    return result


def calc_bond_deriv(y_rate, cpn_freq, years, c_rate, face=100):
    result = 0
    cpn_payout = face * c_rate / cpn_freq

    # print(cpn_payout)
    for i in range(cpn_freq * years):
        time = (i + 1) / cpn_freq

        if i == (cpn_freq * years) - 1:
            val = time * (cpn_payout + face) * math.exp(y_rate * time * -1)
        else:
            val = time * cpn_payout * math.exp(y_rate * time * -1)
        print(f"Cash flow is {val} for time {time} and accumulative is {val + result}")
        result += val
    result *= -1

    return result

def calc_bond_2nd_deriv(y_rate, cpn_freq, years, c_rate, face=100):
    result = 0
    cpn_payout = face * c_rate / cpn_freq

    # print(cpn_payout)
    for i in range(cpn_freq * years):
        time = (i + 1) / cpn_freq

        if i == (cpn_freq * years) - 1:
            val = math.pow(time, 2) * (cpn_payout + face) * math.exp(y_rate * time * -1)
        else:
            val = math.pow(time, 2) * cpn_payout * math.exp(y_rate * time * -1)
        # print(f"Cash flow is {val} for time {time} and accumulative is {val + result}")
        result += val

    return result


def main(price):
    xold = -1000
    xnew = .1
    tol_approx = 1e-6
    while abs(xold - xnew) > tol_approx:
        xold = xnew
        sub = (calc_bond(xnew, 2, 3, .04, 100) - price)/calc_bond_deriv(xnew, 2, 3, .04, 100)
        xnew = xnew - sub
        print(f"xnew is {xnew}")

    # print(xold)
    print(xnew)
    return xnew


if __name__ == "__main__":
    price = 101
    y_rate = main(price)
    print("calc deriv")
    duration = (-1/price) * calc_bond_deriv(y_rate, 2, 3, .04, 100)
    convexity = (1/price) * calc_bond_2nd_deriv(y_rate, 2, 3, .04, 100)

    print(f"Duration is {duration} and convexity is {convexity}")

