#include <iostream>
#include <stdio>
#include <iomanip>
#include <string>
#include <cmath>

void printVector(const std::vector<double>& vec)
{
    for (double element : vec)
    {
        std::cout << element << " ";
    }
    std::cout << std::endl;
}

double IR_a(double B, double C, double t)
{
    return (1.0 / t) * log(100 / B);
}

std::vector<double> IR_b(double r_0, double B, double C, double t, double freq) //B_0 price of 6m 
{
    double num = (B - ((C * 100 / 2) * exp(-t*0.5* r_0)));
    double dom = (100 + (C * freq* 100));

    std::vector<double> rates;

    rates.push_back(r_0);

    rates.push_back(-log(num / dom));

    return rates;
}

std::vector<double> IR_c(std::vector<double> const_rates, double B, double C, double t, double freq, double x0, double tolerance, int max_iter)
{
    int const_size = const_rates.size(); // size of the existing rates

    //std::cout << "Const size: " << const_size << "\n";

    double k = const_rates.back(); // r_1 = constant k

    //std::cout << "k: " << k << "\n";

    std::vector<double> periods = intervals(t, freq); //size = T*freq

    std::vector<double> rates(periods.size(),0.0); //vector of rates from r_0 -> r_t
    
    for (size_t i = 0; i < const_rates.size(); ++i) //Assigning existing rates to new rates vector
    {
        rates[i] = const_rates[i];
    }

    //rates[0] = const_rates[0];
    //rates[1] = const_rates[1];


    int max = (periods.size() - 1);
    

    double b = periods.back(); // last intervals
    //std::cout << "b: " << b << "\n";
    double a = periods[(const_size-1)]; //first rate we don't know, as we know r_0, r_1
    //std::cout << "a: " << a << "\n";
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
        double fx = ((100 + (100 * C * freq)) * exp(-prev * t_end)) - B; //last two terms always follow this formula

        //loop through known rates and add to fx
        for (int i = 0; i < const_size; i++)
        {
            fx += ((C * freq * 100) * exp(-periods[i] * rates[i]));
        }
        /*
        double fx = ((C * freq * 100) * exp(-periods[0] * rates[0])) //r_0
            + ((C * freq * 100) * exp(-periods[1] * k)) + //r_1
            ((100 + (100 * C * freq)) * exp(-prev * periods[max])) - B; //r_2 - B
        */
        
        //loop through remaining rates and add to fx
        for (int i = const_size; i < max; i++)
        {
            double add = (C * freq * 100) * exp(-periods[i] * ((x_coeff[i] * prev) + (k_coeff[i] * k)));
            //std::cout << "fx_add: " << add << "\n";
            fx += add;
        }

       //std::cout << "fx: " << fx << "\n";

        double fdx = -x_coeff[max]  * t_end * ((100 + (100 * C * freq)) * exp(-prev * t_end)); //differential is simply multiplying by the coefficient of x

        //std::cout << "fdx0: " << fdx << "\n";

        for (int i = const_size; i < max; i++)
        {
            double add= -x_coeff[i] * periods[i] * (C * freq * 100) * exp(-periods[i] * ((x_coeff[i] * prev) + (k_coeff[i] * k)));
            //std::cout << "fdx_add: " << add << "\n";
            fdx += add;
        }

        //std::cout << "fdx: " << fdx << "\n";

        curr = prev - (fx / fdx);

        //std::cout << curr << "\n";

        if (iteration > 0)
        {
            double difference = fabs((curr - prev) / curr);
            if (difference < tolerance)
            {
                break; // Convergence reached
            }
        }

        prev = curr;

        

    }

    for (int i = const_size; i <= max; ++i)
    {
        //std::cout << "prev: " << prev << "\n";
        rates[i] = ((x_coeff[i] * prev) + (k_coeff[i] * k));
    }
    /*
    std::cout << "\nperiods: ";
    printVector(periods);

    std::cout << "\nxcoeff: ";
    printVector(x_coeff);

    std::cout << "\nkcoeff: ";
    printVector(k_coeff);


    std::cout << "\nrates: ";
    printVector(rates);
    */


    return rates;
}


int main()
{
    std::cout << "Example from class: \n";

    double six_month = IR_a(99, 0, 0.5);
    std::cout << "\n6-month zero-rate: " << six_month << "\n";

    std::vector<double> one_year_rates = IR_b(six_month, 102, 0.04, 1, 0.5);
    double one_year = one_year_rates.back();
    std::cout << "\n1-year rate: " << one_year << "\n";

    std::vector<double> two_year_rates = IR_c(one_year_rates, 103.5, 0.04, 2.0, 0.5, 0.05, tol, iter_1);
    double two_year = two_year_rates.back();
    std::cout << "\n2-year rate: " << two_year << "\n";

    std::vector<double> five_year_rates = IR_c(two_year_rates, 109, 0.04, 5.0, 0.5, 0.05, tol, iter_1);
    double five_year = five_year_rates.back();
    std::cout << "\n5-year rate: " << five_year << "\n";

    std::cout << "Q7 from exercises: \n";

    double three_month_tbill = IR_a(98.7, 0, 0.25);
    std::cout << "\n3-month zero-rate: " << three_month_tbill << "\n";
   
    double six_month_tbill = IR_a(97.5, 0, 0.5);
    std::cout << "\n6-month zero-rate: " << six_month_tbill << "\n";
   
    std::vector<double> rates_in;
    rates_in.push_back(six_month_tbill); //input has to be vector but it can be a vector of size 1
   
    double price_2y = 100.0 + (5.0 / 32.0);
    std::vector<double> twoyear_rates = IR_c(rates_in, price_2y, 0.04875, 2.0, 0.5, 0.05, tol, iter_1);
    double twoyear = twoyear_rates.back();
    std::cout << "\n2-year bond rate: " << twoyear << "\n";
   
    double price_3y = 100.0 + (5.0 / 32.0);
    std::vector<double> threeyear_rates = IR_c(twoyear_rates, price_3y, 0.04875, 3.0, 0.5, 0.05, tol, iter_1);
    double threeyear = threeyear_rates.back();
    std::cout << "\n3-year bond rate: " << threeyear << "\n";
   
    double price_5y = 99.0 + (22.0 / 32.0);
    std::vector<double> fiveyear_rates = IR_c(threeyear_rates, price_5y, 0.04625, 5.0, 0.5, 0.05, tol, iter_1);
    double fiveyear = fiveyear_rates.back();
    std::cout << "\n5-year bond rate: " << fiveyear << "\n";
   
    double price_10y = 101.0 + (4.0 / 32.0);
    std::vector<double> tenyear_rates = IR_c(fiveyear_rates, price_10y, 0.04875, 10.0, 0.5, 0.05, tol, iter_1);
    double tenyear =tenyear_rates.back();
    std::cout << "\n10-year bond rate: " << tenyear << "\n";

    return 0;
}