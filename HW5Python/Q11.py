import numpy as np

def solve_tridiagonal(n):
    
    a, b = 0, 1  #Domain
    h = (b - a) / n  #Step size
    num_points = n - 1  # (n-1)*(n-1) matrix
    x_values = np.array([i * h for i in range(1,n)])
    
    #Diagonals
    upper_diag = -1*(1+h) * np.ones(num_points - 1)
    main_diag = (h**2 + 2) * np.ones(num_points)  
    lower_diag = -1*(1-h) * np.ones(num_points - 1)

    #print(upper_diag, main_diag, lower_diag)
    
    #b
    b = np.zeros(num_points)
    b[0] = 2*(1-h)  # y(0)
    b[-1] = 3*(1+h)/np.e  # y(1)
    
    #Thomas algorithm
    for i in range(1, num_points):
        factor = lower_diag[i - 1] / main_diag[i - 1]
        main_diag[i] -= factor * upper_diag[i - 1]
        b[i] -= factor * b[i - 1]
    
    #Back substitution
    y = np.zeros(num_points)
    y[-1] = b[-1] / main_diag[-1]
    for i in range(num_points - 2, -1, -1):
        y[i] = (b[i] - upper_diag[i] * y[i + 1]) / main_diag[i]
    

    print(f"{'i':<5} {'x_i':<10} {'y_i':<15}")
    print("-" * 30)
    for i, (x, y) in enumerate(zip(x_values, y), start=1):
         print(f"{i:<5} {x:<10.3f} {y:<15.10f}")
    
    

# Solve for n = 8
n = 8
print("\nn=8\n")
solve_tridiagonal(n)
n = 16
print("\nn=16\n")
solve_tridiagonal(n)
n = 32
print("\nn=32\n")
solve_tridiagonal(n)
n = 64
print("\nn=64\n")
solve_tridiagonal(n)

