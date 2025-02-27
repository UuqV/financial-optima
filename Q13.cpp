#include <iostream>
#include <cmath>
#include <iomanip>
#include <vector>

//Instantaneous rate
double r(double t)
{
    return 0.05 / (1 + 2 * exp(-pow(1 + t, 2)));
}

//Simpson's rule for calculating integral from 0 to t
double integrate_r(double t, int n)
{
    if (n % 2 != 0) n++;  //n must be even

    const double h = t / n; //h or delta_x
    double sum = r(0) + r(t); //first and last term of the bracket

    //iterate from 1 to n-1
    for (int i = 1; i < n; i++)
    {
        double x_i = i * h;
        double coefficient = (i % 2 == 0) ? 2 : 4; //coeff 2 if i is even, 4 if odd
        sum += coefficient * r(x_i); //adding each term to the sum
    }

    return (h / 3) * sum;
}

//Discount factor
double discount_factor(double t, int n)
{
    return exp(-integrate_r(t, n));
}

//Iterations of the discount factor
std::vector<double> discount_factor_iters(double t, double tol, int max_iters)
{
    std::vector<double> approximations; //vector to store approximations
    int n = 4; //start with 4 intervals
    double prev = 0.0;
    double current = 0.0;

    for (int i = 0; i < max_iters; i++)
    {
        current = discount_factor(t, n);
        approximations.push_back(current);

        //checking convergence after firts iteration
        if (i > 0)
        {
            double diff = fabs((current - prev) / current); //check difference

            if (diff < tol) { break;} //finish if tolerance reached
        }

        prev = current; //assigning for next iteration
        n *= 2; // Double the number of intervals
    }

    return approximations;
}

int main() {
    //timestamps in years
    const double t_6m = 0.5;
    const double t_1y = 1.0;
    const double t_18m = 1.5;
    const double t_2y = 2.0;

    //Parameters
    const double tol_1 = 1e-6; //6dp for 6month, 1y, 18month
    const double tol_2 = 1e-8; //8dp for 2y
    const int max_iters = 15;

    //Part i)
    std::cout << "13i) Integration iterations:" << "\n";

    //6-month df
    std::cout << "\n6-month discount factor convergence:" << "\n";
    std::vector<double> df_6m_approx = discount_factor_iters(t_6m, tol_1, max_iters);
    std::cout << std::fixed << std::setprecision(10);
    for (int i = 0; i < df_6m_approx.size(); i++)
    {
        int intervals = 4 * pow(2, i);
        std::cout << "n = " << std::setw(6) << intervals << " : " << df_6m_approx[i];
        std::cout << std::fixed << "\n";
    }
    double df_6m = df_6m_approx.back(); //elements are pushed to the back, so most current approximation is at the end

    //1-year df
    std::cout << "\n1-year discount factor convergence:" << "\n";
    std::vector<double> df_1y_approx = discount_factor_iters(t_1y, tol_1, max_iters);
    for (int i = 0; i < df_1y_approx.size(); i++)
    {
        int intervals = 4 * pow(2, i);
        std::cout << "n = " << std::setw(6) << intervals << " : " << df_1y_approx[i];
        std::cout << std::fixed << "\n";
    }
    double df_1y = df_1y_approx.back();

    //18-month df
    std::cout << "\n18-month discount factor convergence:" << "\n";
    std::vector<double> df_18m_approx = discount_factor_iters(t_18m, tol_1, max_iters);
    for (int i = 0; i < df_18m_approx.size(); i++)
    {
        int intervals = 4 * pow(2, i);
        std::cout << "n = " << std::setw(6) << intervals << " : " << df_18m_approx[i];
        std::cout << std::fixed << "\n";
    }
    double df_18m = df_18m_approx.back();

    //2-yar df
    std::cout << "\n2-year discount factor convergence:" << "\n";
    std::vector<double> df_2y_approx = discount_factor_iters(t_2y, tol_2, max_iters);
    for (int i = 0; i < df_2y_approx.size(); i++)
    {
        int intervals = 4 * pow(2, i);
        std::cout << "n = " << std::setw(6) << intervals << " : " << df_2y_approx[i];
        std::cout << std::fixed << "\n";
    }
    double df_2y = df_2y_approx.back();

    std::cout << "\nDiscount Factors:" << "\n";
    std::cout << std::setprecision(6) << "6 months: " << df_6m << "\n";
    std::cout << std::setprecision(6) << "1 year: " << df_1y << "\n";
    std::cout << std::setprecision(6) << "18 months: " << df_18m << "\n";
    std::cout << std::setprecision(8) << "2 years: " << df_2y << "\n";

    //Part(ii)
    //Bond parameters
    double coupon_rate = 0.05;
    double face_value = 100.0;
    double coupon_amount = face_value * coupon_rate / 2; // /2 for semi-annual

    //Calculating present value of each cash flow
    double pv_coupon_6m = coupon_amount * df_6m;
    double pv_coupon_1y = coupon_amount * df_1y;
    double pv_coupon_18m = coupon_amount * df_18m;
    double pv_coupon_2y = coupon_amount * df_2y;
    double pv_face_value = face_value * df_2y;

    //Total present value/(bond price)
    double bond_price = pv_coupon_6m + pv_coupon_1y + pv_coupon_18m + pv_coupon_2y + pv_face_value;

    std::cout << "\nBond Price: $" << std::setprecision(6) << bond_price << "\n";

    return 0;
}
