import math 
import time 


def rate_disc(cash_flow_time):
    return math.exp(-.025 * cash_flow_time)

def price_bond( t_cash_flow, v_cash_flow):

    retVal = 0
    for i in range(len(t_cash_flow)):
        amt = v_cash_flow[i] * rate_disc(t_cash_flow[i])
        retVal += amt
    return retVal 

def get_duration(t_cash_flow, v_cash_flow, price):
    retVal = 0
    for i in range(len(t_cash_flow)):
        amt = t_cash_flow[i] * v_cash_flow[i] * rate_disc(t_cash_flow[i])
        retVal += amt
    
    return retVal/price

def get_convexity(t_cash_flow, v_cash_flow, price):
    retVal = 0
    for i in range(len(t_cash_flow)):
        amt = math.pow(t_cash_flow[i],2) * v_cash_flow[i] * rate_disc(t_cash_flow[i])
        retVal += amt
    
    return retVal/price

if __name__ == "__main__":
    
    cf_time = [1/12, 7/12, 13/12, 19/12]
    cf_amt = [2, 2, 2, 102]
    price = price_bond(cf_time, cf_amt)
    print(price)

    print("Get Duration")
    dur = get_duration(cf_time, cf_amt, price)
    print(dur)

    print("Get Convexity")
    conv = get_convexity(cf_time, cf_amt, price)
    print(conv)

