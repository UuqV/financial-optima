import math
import scipy

vol = .2
spot = 50
strike = 45
time = 9/12
interest = .02
divs = .01
bs_price = 8
tol_approx = 1e-6


def print_ascii_table(float_tuple_list):
    print(f"{'Index':<8}{'Volatility':<15}{'BS Value'}")
    print('-' * 40)  # Line separator

    # Print each index and corresponding rounded float values from the tuple
    for i, (num1, num2) in enumerate(float_tuple_list, start=1):
        print(f"{i:<8}{num1:.8f}    {num2:.8f}")


def d1(spot, strike, interest, vol, time, divs):
    num = math.log(spot / strike) + (interest - divs + (math.pow(vol, 2) / 2)) * time
    denom = vol * math.sqrt(time)
    return num / denom


def d2(vol, time, d_1):
    return d_1 - (vol * math.sqrt(time))


def call(vol, spot, strike, time, interest, divs, output=False):
    d_1 = d1(spot, strike, interest, vol, time, divs)
    d_2 = d2(vol, time, d_1)
    bs_call = spot * math.exp(-divs * time) * scipy.stats.norm.cdf(d_1) - strike * math.exp(-interest * time) * scipy.stats.norm.cdf(d_2)
    if output:
        print(f"Black Scholes Call for vol {vol} value is {bs_call}")
    return bs_call


def put(vol, spot, strike, time, interest, divs, output=False):
    d_1 = d1(spot, strike, interest, vol, time, divs)
    d_2 = d2(vol, time, d_1)
    bs_put = strike * math.exp(-interest * time) * scipy.stats.norm.cdf(-d_2) - spot * math.exp(-divs * time) * scipy.stats.norm.cdf(-d_1)
    if output:
        print(f"Black Scholes Call for vol {vol} value is {bs_put}")
    return bs_put

# Same for calls and puts
def vega(sig, S, K, r, T, q):
    d_1 = d1(S, K, r, sig, T, q)
    ed_1 = scipy.stats.norm.pdf(d_1)
    v = S * math.exp(-q * T) * ed_1 * math.sqrt(T)
    return v


def delta_call(spot, strike, interest, vol, time, divs):
    d_1 = d1(spot, strike, interest, vol, time, divs)
    return math.exp(-time * divs) * scipy.stats.norm.cdf(d_1)


def delta_put(spot, strike, interest, vol, time, divs):
    d_1 = d1(spot, strike, interest, vol, time, divs)
    return math.exp(-time * divs) * (scipy.stats.norm.cdf(d_1) - 1)


# Same for calls and puts
def gamma(spot, strike, interest, vol, time, divs):
    d_1 = d1(spot, strike, interest, vol, time, divs)
    exp_term = math.exp(-divs * time) / (spot * vol * math.sqrt(time))
    return exp_term * scipy.stats.norm.pdf(d_1)


def theta_call(spot, strike, interest, vol, time, divs):
    d_1 = d1(spot, strike, interest, vol, time, divs)
    d_2 = d2(vol, time, d_1)

    # First part of the formula (S * sigma * pdf(d1) / (2 * sqrt(T)) * e^(-qT))
    term_1 = -spot * vol * math.exp(-divs * time) * scipy.stats.norm.pdf(d_1) / (2 * math.sqrt(time))

    # Second part of the formula (-r * K * e^(-rT) * N(d2))
    term_2 = -interest * strike * math.exp(-interest * time) * scipy.stats.norm.cdf(d_2)

    # Third part of the formula (q * S * e^(-qT) * N(d1))
    term_3 = divs * spot * math.exp(-divs * time) * scipy.stats.norm.cdf(d_1)

    # Total theta for call option
    return term_1 + term_2 + term_3


def theta_put(spot, strike, interest, vol, time, divs):
    d_1 = d1(spot, strike, interest, vol, time, divs)
    d_2 = d2(vol, time, d_1)

    # First part of the formula (S * sigma * pdf(d1) / (2 * sqrt(T)) * e^(-qT))
    term_1 = -spot * vol * math.exp(-divs * time) * scipy.stats.norm.pdf(d_1) / (2 * math.sqrt(time))

    # Second part of the formula (r * K * e^(-rT) * N(-d2))
    term_2 = interest * strike * math.exp(-interest * time) * scipy.stats.norm.cdf(-d_2)

    # Third part of the formula (-q * S * e^(-qT) * N(-d1))
    term_3 = -divs * spot * math.exp(-divs * time) * scipy.stats.norm.cdf(-d_1)

    # Total theta for put option
    return term_1 + term_2 + term_3


def rho_call(spot, strike, interest, vol, time, divs):
    d_1 = d1(spot, strike, interest, vol, time, divs)
    d_2 = d2(vol, time, d_1)

    return strike * time * math.exp(-interest * time) * scipy.stats.norm.cdf(d_2)


def rho_put(spot, strike, interest, vol, time, divs):
    d_1 = d1(spot, strike, interest, vol, time, divs)
    d_2 = d2(vol, time, d_1)

    return -strike * time * math.exp(-interest * time) * scipy.stats.norm.cdf(-d_2)


def wrt_K_call(spot, strike, interest, vol, time, divs):
    d_1 = d1(spot, strike, interest, vol, time, divs)
    d_2 = d2(vol, time, d_1)

    e_term = -math.exp(-interest * time) * scipy.stats.norm.cdf(d_2)
    constant = 1/(strike * vol * math.sqrt(time))
    strike_term = strike * math.exp(-interest * time) * scipy.stats.norm.pdf(d_2)
    spot_term = spot * math.exp(-divs * time) * scipy.stats.norm.pdf(d_1)
    return e_term + constant * (strike_term - spot_term)

def wrt_K_put(spot, strike, interest, vol, time, divs):
    d_1 = d1(spot, strike, interest, vol, time, divs)
    d_2 = d2(vol, time, d_1)

    e_term = math.exp(-interest * time) * scipy.stats.norm.cdf(-d_2)
    constant = 1/(strike * vol * math.sqrt(time))
    strike_term  = strike * math.exp(-interest * time) * scipy.stats.norm.pdf(-d_2)
    spot_term = spot * math.exp(-divs * time) * scipy.stats.norm.pdf(d_1)
    return e_term + constant * (strike_term - spot_term)


def question_1(vol, spot, strike, time, interest, divs, bs_price, tol_approx, func="C"):
    if func == "C":
        func = call
    elif func == "P":
        func = put

    prev = -1000
    curr = vol
    vol_bs = [(curr, func(curr, spot, strike, time, interest, divs))]

    while abs(prev - curr) > tol_approx:
        prev = curr

        sub = (func(curr, spot, strike, time, interest, divs) - bs_price)/vega(curr, spot, strike, interest, time, divs)
        curr = curr - sub

        vol_bs.append((curr, func(curr, spot, strike, time, interest, divs)))

    print_ascii_table(vol_bs)


def question_4(vol, spot, strike, time, interest, divs, tol_approx):
    prev = -1000 
    curr = strike
    strike_record = [(curr, put(vol, spot, curr, time, interest, divs))]
    while abs(curr - prev) > tol_approx:
        prev = curr 

        num = put(vol, spot, curr, time, interest, divs) - curr + spot
        denom = wrt_K_put(spot, curr, interest, vol, time, divs) - 1

        curr = curr - (num/denom)
        strike_record.append((curr, put(vol, spot, curr, time, interest, divs)))
    print_ascii_table(strike_record)

        

if __name__ == "__main__":
    question_1(vol, spot, strike, time, interest, divs, bs_price, tol_approx, func="C")
    vol = .25
    spot = 50
    strike = 50
    time = .5
    interest = .03
    divs = .01
    question_4(vol, spot, strike, time, interest, divs, tol_approx)
