import math

# TODO: Input variables
face = 100
cpn_freq = 6/12
cpn_rate = .04
maturity = 28/12

def risk_free_rate(x):
    # TODO: Input Formula calculation 
    return .015 + (1 + 2 * math.pow(x, 2))/(100 + 100 * math.pow(x, 2))


cash_flows = [] # time, cash flow, discount rate, pv

calc_time = maturity
cpn_payment = cpn_rate * face * cpn_freq
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
print(cash_flows)
price = sum(cash_flow[3] for cash_flow in cash_flows)
all_cpns = [cash_flow[0] for cash_flow in cash_flows]

#___________________________ Copy of Bootstrapping inputs

# TODO: Input variables 
bonds = [
    (28/12, cpn_rate * 100, price, True)
]
initial_r = .1
result = [(0, risk_free_rate(0))] # Time yield 
newton_output = []
OFFSET = -2/12

#__________________________ Copy of Bootstrapping


def print_ascii_table(float_list):
    print(f"{'Index':<8}{'Guesses'}")
    print('-' * 20)  # Line separator

    # Print each index and corresponding rounded float value
    for i, num in enumerate(float_list, start=1):
        print(f"{i:<8}{num:.6f}")

def price_guess(guess_r, cpn_total_payout, cpn_payment, bond, result_correct=False):
    time, _, recorded_price, write_res = bond
    last_bond_time, last_bond_yield = result[-1]
    cpn_time = last_bond_time + cpn_freq + OFFSET

    while cpn_time < time:
        # Calculate the time between coupon and last bond to offset it and
        # guessing bond to last bond for slope
        cpn_time_diff = cpn_time - last_bond_time
        guess_time_diff = time - last_bond_time
        slope = (guess_r - last_bond_yield) / guess_time_diff  # linear slopt of the yield
        slope_time_adjusted = last_bond_yield + (slope * cpn_time_diff)
        if write_res:
            print(f"Slope time adjusted/yield for {cpn_time} is {slope_time_adjusted}")
        if result_correct:
            result.append((cpn_time, slope_time_adjusted))

        cpn_total_payout += cpn_payment * math.exp(-1 * cpn_time * slope_time_adjusted)
        cpn_time += cpn_freq

    total_payout = cpn_total_payout + (face + cpn_payment) * math.exp(-1 * time * guess_r)

    return total_payout - recorded_price


def price_deriv(guess_r, cpn_payment, bond):
    time, _, _, _ = bond
    last_bond_time, last_bond_yield = result[-1]
    cpn_time = last_bond_time + cpn_freq + OFFSET

    deriv_slope = 1 / (time - last_bond_time)

    cpn_deriv = 0

    while cpn_time < time:

        # Calculate the time between coupon and last bond to offset it
        # and guessing bond to last bond for slope
        cpn_time_diff = cpn_time - last_bond_time
        guess_time_diff = time - last_bond_time
        slope = (guess_r - last_bond_yield) / guess_time_diff
        slope_time_adjusted = last_bond_yield + (slope * cpn_time_diff)

        base_payment = cpn_payment * math.exp(-1 * cpn_time * slope_time_adjusted)

        # Calculate derivative of exponential discount
        deriv_of_exp = -cpn_time * deriv_slope * cpn_time_diff

        cpn_deriv += deriv_of_exp * base_payment
        cpn_time += cpn_freq

    last_payment = -time * (face + cpn_payment) * math.exp(-1 * time * guess_r)
    # print(f"Calculated price for {guess_r} is {cpn_deriv + last_payment}\n")
    return cpn_deriv + last_payment


prev_time = None
for bond in bonds:
    time, cpn_rate, price, write_res = bond
    cpn_total_payout = 0
    cpn_payment = face * (cpn_rate * cpn_freq / 100)

    for i in result:
        prev_time, prev_yield = i
        if prev_time == 0:
            continue
        cpn_total_payout += cpn_payment * math.exp(-1 * prev_time * prev_yield)

    if time == .5 or (len(result) > 0 and result[-1][0] == time - cpn_freq):
        r = math.log((price - cpn_total_payout) / (face + cpn_payment)) / (-1 * time)
        result.append((time, r))
        continue

    guess_old = -1000
    guess_r = initial_r
    tol_approx = 1e-6
    print(f"Beginning estimation for {time} with {guess_r}")
    while abs(guess_old - guess_r) > tol_approx:
        guess_old = guess_r
        sub = price_guess(guess_r, cpn_total_payout, cpn_payment, bond) / price_deriv(guess_r, cpn_payment, bond)
        guess_r = guess_r - sub
        if bond[3]:
            newton_output.append(guess_r)
        print(f"New guess for {time} is {guess_r}")

    price_guess(guess_r, cpn_total_payout, cpn_payment, bond, True)
    result.append((time, guess_r))
    print(f"Bond at time {time} has rate: {guess_r}")
    if bond[3]:
        print_ascii_table(newton_output)

print(result)
print(f"PRICE WAS {price}")