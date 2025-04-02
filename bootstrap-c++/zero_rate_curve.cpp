#include <iostream>
#include <stdio.h>
#include <iomanip>
#include <string>
#include <cmath>
#include <vector>
#include <algorithm>

void printVector(const std::vector<double>& vec)
{
    for (double element : vec)
    {
        std::cout << element << " ";
    }
    std::cout << std::endl;
}

//Function to generate time intervals for bonds based on duration and frequency of payments per year
std::vector<double> intervals(double time, double frequency)
{
    std::vector<double> intervals;
    double currentTime = frequency;

    while (currentTime <= time)
    {
        intervals.push_back(currentTime);
        currentTime += frequency;
    }

    return intervals;
}
//Function to calculate zero rate for a bond with only one payment
double IR_a(double B, double C, double t)
{
    return (1.0 / t) * log(100 / B);
}
//Function to calculate zero rate for a bond with 2 payments
std::vector<double> IR_b(double r_0, double B, double C, double t, double freq) 
{
    double num = (B - ((C * 100 / 2) * exp(-t*0.5* r_0)));
    double dom = (100 + (C * freq* 100));

    std::vector<double> rates;

    rates.push_back(r_0);

    rates.push_back(-log(num / dom));

    return rates;
}
//Function to calculate zero rate for a bond with more than 2 payments
std::vector<double> IR_c(std::vector<double> const_rates, double B, double C, double t, double freq, double x0, double tolerance, int max_iter)
{
    int const_size = const_rates.size(); // size of the existing rates

    double k = const_rates.back(); // last known rate = constant k

    std::vector<double> periods = intervals(t, freq); //size = T*freq

    std::vector<double> rates(periods.size(),0.0); //vector of rates from r_0 -> r_t
    
    for (size_t i = 0; i < const_rates.size(); ++i) //Assigning existing rates to new rates vector
    {
        rates[i] = const_rates[i];
    }

    int max = (periods.size() - 1);
    

    double b = periods.back(); // last intervals;
    double a = periods[(const_size-1)]; //first rate we don't know, as we know r_0, r_1
    double length = b - a; // b-a

    std::vector<double> x_coeff;
    std::vector<double> k_coeff;
    for (int i = 0; i < periods.size(); i++)
    {
        if(i<const_size)
        {
            x_coeff.push_back(0); //0 all coefficients for the constants as rates known
        }
        else
        {
            x_coeff.push_back((periods[i] - a) / length);
        }
    }
    for (int i = 0; i < periods.size(); i++)
    {
        if (i < const_size)
        {
            k_coeff.push_back(0); //0 all coefficients for the constants as rates known
        }
        else
        {
            k_coeff.push_back((b - periods[i]) / length);
        }
    }

    double t_end = periods.back();

    double prev = x0;
    double curr = 0.0;

    for (int iteration = 0; iteration < max_iter; ++iteration)
    {
        double fx = ((100 + (100 * C * freq)) * exp(-prev * t_end)) - B; //last term always follow this formula

        //loop through known rates and add to fx
        for (int i = 0; i < const_size; i++)
        {
            fx += ((C * freq * 100) * exp(-periods[i] * rates[i]));
        }
        
        
        //loop through remaining rates and add to fx
        for (int i = const_size; i < max; i++)
        {
            double add = (C * freq * 100) * exp(-periods[i] * ((x_coeff[i] * prev) + (k_coeff[i] * k)));
            fx += add;
        }

        double fdx = -x_coeff[max]  * t_end * ((100 + (100 * C * freq)) * exp(-prev * t_end)); //differential is simply multiplying by the coefficient of x


        for (int i = const_size; i < max; i++)
        {
            double add= -x_coeff[i] * periods[i] * (C * freq * 100) * exp(-periods[i] * ((x_coeff[i] * prev) + (k_coeff[i] * k)));
            fdx += add;
        }

        curr = prev - (fx / fdx);

        if (iteration > 0)
        {
            double difference = fabs((curr - prev) / curr);
            if (difference < tolerance)
            {
                break; // Convergence reached
            }
        }

        prev = curr;

        std::cout << "Iteration : " << iteration << "  Rate: " << curr << "\n";

    }

    //calculate the rates for the remaining periods using the coefficients betwween a and b
    for (int i = const_size; i <= max; ++i)
    {
        rates[i] = ((x_coeff[i] * prev) + (k_coeff[i] * k));
    }


    return rates; //returns a vector of the rates from (00) to (0,t)
}


int main()
{
    
    double tol = 0.0000000000001;
    int iter_1 = 20;
    
    double overnight = 0.05; //overnight rate

    //6-month
    //input price = 97.5, coupon rate = 0, time = 0.5years (6months)
    double six_month = IR_a(97.5, 0, 0.5); 
    std::cout << "6-month zero-rate: " << six_month << "\n";

    //1-year
    //input price = 100, coupon rate = 0.05 (5%), time = 1year, frequency = 0.5 (6months)
    std::vector<double> one_year_rates = IR_b(six_month, 100, 0.05, 1, 0.5); 
    double one_year = one_year_rates.back();
    std::cout << "\n1-year rate: " << one_year << "\n";
    
    //3-year
    std::cout << "\n3-year rate convergence: \n";
    //input price = 102, coupon rate = 0.05 (5%), time = 3years, frequency = 0.5 (6months), inital guess= 0.05, tolerance, max_iterations
    std::vector<double> three_year_rates = IR_c(one_year_rates, 102, 0.05, 3.0, 0.5, 0.05, tol, iter_1);  
    double three_year = three_year_rates.back();
    std::cout << "\n3-year rate: " << three_year << "\n";

	std::vector<double> three_year_periods = intervals(3.0, 0.5);

	std::cout << "r(0,0) = " << overnight << " (overnight rate)\n";

	for (int i = 0; i < three_year_periods.size(); i++)
	{
		std::cout << "r(0," << three_year_periods[i] << ") = " << three_year_rates[i] << "\n";
	}
    
    //5-year
    std::cout << "\n5-year rate convergence: \n";
    //input price = 104, coupon rate = 0.05 (5%), time = 5years, frequency = 0.5 (6months), inital guess= 0.05, tolerance, max_iterations
    std::vector<double> five_year_rates = IR_c(three_year_rates, 104, 0.06, 5.0, 0.5, 0.05, tol, iter_1);
    double five_year = five_year_rates.back();
    std::cout << "\n5-year rate: " << five_year << "\n";

    std::vector<double> five_year_periods = intervals(5.0, 0.5);

    std::cout << "r(0,0) = " << overnight << " (overnight rate)\n";

    for (int i = 0; i < five_year_periods.size(); i++)
    {
        std::cout << "r(0," << five_year_periods[i] << ") = " << five_year_rates[i] << "\n";
    }

    return 0;
}