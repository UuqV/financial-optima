import math


def risk_free_rate(x):
    # TODO: Input Formula calculation
    return .015 + (1 + 2 * math.pow(x, 2))/(100 + 100 * math.pow(x, 2))


def calculate_cash_flows(calc_time, cpn_payment):
    cash_flows = [] # time, cash flow, discount rate, pv
    while calc_time > 0:
        if calc_time == maturity:
            cash_flow = face + cpn_payment
        else:
            cash_flow = cpn_payment

        discount = risk_free_rate(calc_time)
        pv = cash_flow * math.exp(-calc_time * discount)

        cash_flows.append((calc_time, cash_flow, discount, pv))
        calc_time -= cpn_freq
    cash_flows.reverse()
    return cash_flows


def sum_up_cashflows(yield_guess, price, cash_flows):
    pv_sum = 0
    for time, cash_flow, _, _ in cash_flows:
        pv_sum += cash_flow * math.exp(-yield_guess * time)
    return pv_sum - price

def sum_up_deriv(yield_guess, cash_flows):
    # Duplicate of bond_first_deriv
    pv_sum = 0
    for time, cash_flow, _, _ in cash_flows:
        pv_sum += time * cash_flow * math.exp(-yield_guess * time)

    return pv_sum

def bond_yield_calculation(initial_r, price, cash_flows, tol):
    # cash_flows = time, cash flow, discount rate, pv
    xold = -1000
    xnew = initial_r
    while abs(xold-xnew) > tol:
        xold = xnew
        num = sum_up_cashflows(xnew, price, cash_flows)
        denom = sum_up_deriv(xnew, cash_flows)
        xnew = xnew + num/denom
        print(xnew)
    return xnew


# bootstrap_zerorate(initial_r, face, cpn_freq, result, bonds, OFFSET, newton_output)
# print(f"Now result is {result}")
def bond_second_deriv(cash_flows, calc_yield):
    pv_sum = 0
    for time, cash_flow, _, _ in cash_flows:
        pv_sum += math.pow(time, 2) * cash_flow * math.exp(-calc_yield * time)
    return pv_sum

def print_ascii_table(time_rate_list):
    # Header
    header = f"{'Time':>10} | {'Rate':>20}"
    separator = "-" * len(header)
    print(header)
    print(separator)
    
    # Rows
    for time, rate in time_rate_list:
        print(f"{time:10.8f} | {rate:20.8f}")


def num_calcs(cash_flows, price, calc_yield):
    deriv = sum_up_deriv(calc_yield, cash_flows)
    duration = (1/price) * deriv

    second_deriv = bond_second_deriv(cash_flows, calc_yield)
    convexity = (1/price) * second_deriv

    dollar_duration = -deriv

    dollar_convexity = second_deriv

    dv01 = dollar_duration / 10_000

    print(f"Price is {price}")

    print(f"Duration is {duration}")
    print(f"Convexity is {convexity}")

    print(f"Dollar Duration is {dollar_duration}")
    print(f"Dollar Convexity is {dollar_convexity}")

    print(f"DV01 is {dv01}")


if __name__ == "__main__":
    face = 100
    cpn_freq = 6 / 12
    cpn_rate = .04
    maturity = 28 / 12
    calc_time = maturity
    cpn_payment = cpn_rate * face * cpn_freq
    cash_flows = calculate_cash_flows(calc_time, cpn_payment)
    initial_r = .1

    price = sum(cash_flow[3] for cash_flow in cash_flows)
    calc_yield = bond_yield_calculation(initial_r, price, cash_flows, tol=1e-6)
    num_calcs(cash_flows, price, calc_yield)

# Compute the modified duration, dollar duration, DV01, convexity, and
# dollar convexity of the bond.