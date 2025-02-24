import math 
import time 


def ten_zero_rate_disc(cash_flow_time):
    r_zero = .05 + .005 * math.sqrt(1+cash_flow_time)
    disc = math.exp((-1 * cash_flow_time) * r_zero)
    return disc 

def price_bond_inst_10(n, t_cash_flow, v_cash_flow):

    retVal = 0
    for i in range(n):
        disc = ten_zero_rate_disc(t_cash_flow[i])
        retVal += (v_cash_flow[i] * disc)
    return retVal 

#_________________________________________________
def eleven_zero_rate_disc(cash_flow_time):
    r_zero = .02 + (cash_flow_time/(200-cash_flow_time))
    disc = math.exp((-1 * cash_flow_time) * r_zero)
    return disc 

def price_bond_inst_11(n, t_cash_flow, v_cash_flow):
    retVal = 0
    for i in range(n):
        disc = eleven_zero_rate_disc(t_cash_flow[i])
        retVal += (v_cash_flow[i] * disc)
    return retVal 

if __name__ == "__main__":
    # 10A
    cf_time = [6/12, 1, 18/12, 2]
    cf_amt = [3.5, 3.5, 3.5, 103.5]
    x = price_bond_inst_10(4, cf_time, cf_amt)
    print(x)

    # 10b
    cf_time = [1/12, 7/12, 13/12, 19/12]
    cf_amt = [3.5, 3.5, 3.5, 103.5]
    x = price_bond_inst_10(4, cf_time, cf_amt)
    print(x)

    # 11a
    cf_time = [7/12, 19/12]
    cf_amt = [4, 104]
    x = price_bond_inst_11(2, cf_time, cf_amt)
    print(x) 

    # 11b 
    cf_time = [1/12, 7/12, 13/12, 19/12]
    cf_amt = [2, 2, 2, 102]
    x = price_bond_inst_11(4, cf_time, cf_amt)
    print(x)

    # 11c
    cf_time = [1/12, 4/12, 7/12, 10/12, 13/12, 16/12, 19/12]
    cf_amt = [1, 1, 1, 1, 1, 1, 101]
    x = price_bond_inst_11(7, cf_time, cf_amt)
    print(x)
