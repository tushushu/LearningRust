#include <chrono>
#include <iostream>
#include <random>
using namespace std;

int main()
{
    // Duration of C++: 10.104 ms
    std::mt19937 gen(100);
    uniform_real_distribution<> dis(-1.0, 1.0);
    int n = 10000000;
    vector<float> vec;
    for (int i = 0; i < n; ++i)
    {
        vec.push_back(dis(gen));
    }

    auto start = chrono::steady_clock::now();
    float result = 0.0;
    for (int i = 0; i < n; ++i)
    {
        result += vec[i];
    }
    auto end = chrono::steady_clock::now();
    auto duration = end - start;

    cout << "Duration of C++: " << chrono::duration<double, milli>(duration).count() << " ms" << endl;
    cout << "The result is: " << result;
}
