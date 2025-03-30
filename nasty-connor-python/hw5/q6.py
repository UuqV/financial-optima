import math

bonds = [
    (.5, 0, 97.5, False),
    (1, 5, 100, False),
    (3, 5, 102, True ),
    (5, 7, 104, False),
]
cpn_freq = .5
max_time = 5
face = 100
result = [] # time, yield

def price_guess(guess_r, cpn_total_payout, cpn_payment, bond):
    time, _, recorded_price, write_res = bond
    new_cpn_iter = result[-1][0] + cpn_freq
    for cpn_time in range(int(new_cpn_iter * 2), int(time * 2), int(cpn_freq * 2)):
        cpn_time /= 2
        cpn_total_payout += cpn_payment * math.exp(-1 * cpn_time * guess_r)
    total_payout = cpn_total_payout + (face + cpn_payment) * math.exp(-1 * time * guess_r)

    return total_payout - recorded_price
    
def price_deriv(guess_r, cpn_payment, bond):
    time, _, recorded_price, write_res = bond
    new_cpn_iter = result[-1][0] + cpn_freq
    prev_time, prev_yield = result[-1]
    deriv_slope = 1 / (time - prev_time)
    slope = (guess_r - prev_yield) / (time - prev_time)
    cpn_deriv = 0

    for cpn_time in range(int(new_cpn_iter * 2), int(time * 2), int(cpn_freq * 2)):
        cpn_time /= 2

        time_diff_previous = new_cpn_iter - prev_time
        deriv = -1 * time_diff_previous * cpn_time * deriv_slope

        cpn_deriv += deriv * cpn_payment * math.exp(-1 * cpn_time * slope * time_diff_previous)

    last_payment = -time  * (face + cpn_payment) * math.exp(-1 * time * guess_r)
    return cpn_deriv + last_payment

prev_time = None
for bond in bonds:
    time, cpn_rate, recorded_price, write_res = bond
    # TODO: Theoretically he could have it be a semi-annual bond but then offset it and ask the price at t = 3 months
    cpn_total_payout = 0
    cpn_payment = face * (cpn_rate * cpn_freq / 100)
    
    for i in result:
        prev_time, prev_yield = i
        cpn_total_payout += cpn_payment * math.exp(-1 * prev_time * prev_yield)

    if time == .5 or result[-1][0] == time - cpn_freq:
        r = math.log( (recorded_price - cpn_total_payout) / (face + cpn_payment) ) / ( -1 * time)
        result.append((time, r))
        continue

    guess_old = -1000
    guess_r = .05
    tol_approx = 1e-6
    while abs(guess_old - guess_r) > tol_approx:
        guess_old = guess_r
        sub =  price_guess(guess_r, cpn_total_payout, cpn_payment, bond)/ price_deriv(guess_r, cpn_payment, bond)
        guess_r = guess_r - sub
        print(guess_r)
    print(guess_r)
