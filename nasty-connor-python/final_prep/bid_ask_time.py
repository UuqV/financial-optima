import math

p_ask = 5.25 
p_bid = 4.75

s_ask = 51.2
s_bid = 50.85

c_ask = 6
c_bid = 5.5

strike = 50
interest = .02
time = .5


def interest_valued_too_much():
    #pa + sa - cb < ke^rt -> Arbitrage exists
    asset = p_ask + s_ask - c_bid 
    time_value = strike * math.exp(-interest * time)

    print(f"Asset Value: Pa {p_ask} + S_a {s_ask} - C_b {c_bid} = {asset}")
    print(f"Time Value: {strike} * e^{-interest} * {time} = {time_value}")
    if asset < time_value:
        arbitrage = asset * math.exp(interest * time)
        print(f"Arbitrage is {arbitrage}, what we get is {arbitrage - strike}")
        print("Sell call, buy put and asset, make difference between what you bought it for and strike")
    else:
        print("Interest is not valued too much ")
    print("\n\n")
    

def interest_not_valued_enough():
    #pb + sb - ca > ke^rt -> Arbitrage exists
    asset = p_bid + s_bid - c_ask
    time_value = strike * math.exp(-interest * time)
    print(f"Asset Value: Pb {p_bid} + Sb {s_bid} - Ca {c_ask} = {asset}")
    print(f"Time Value: {strike} * e^{-interest} * {time} = {time_value}")
    if asset > time_value:
        arbitrage = asset * math.exp(interest * time)
        print(f"Arbitrage is {arbitrage}, what we get is {arbitrage - strike}")
        print("Buy call, sell put and asset, invest at r")
    else:
        print("Interest is not undervalued")

if __name__ == "__main__":
    interest_valued_too_much()
    interest_not_valued_enough()
