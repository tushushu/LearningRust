#include <chrono>
#include <iostream>
#include <random>
using namespace std;

int main()
{
    // Duration of C++: 9.256 ms
    auto start = chrono::steady_clock::now();

    std::mt19937 gen(100);
    uniform_real_distribution<> dis(-1.0, 1.0);
    float result = 0.0;
    int m = 1000;
    int n = 1000;
    for (int i = 0; i < m; ++i)
    {
        for (int j = 0; j < n; ++j)
        {
            result += dis(gen);
        }
    }
    auto end = chrono::steady_clock::now();
    auto duration = end - start;
    cout << "Duration of C++: " << chrono::duration<double, milli>(duration).count() << " ms" << endl;
}
