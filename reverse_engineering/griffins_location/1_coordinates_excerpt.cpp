#include <string>
#include <random>
#include <ctime>
#include <iostream>

void print4(std::string* arg1, int32_t arg2)
{
    std::string alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{}|;:,.<>?/";
    std::mt19937 random_source(time(nullptr));

    std::uniform_int_distribution<> rng(0, alphabet.size() - 1);

    *arg1 = "";
    for (int i = 0; i < arg2; i += 1) {
        *arg1 += alphabet[rng(random_source)];
    }
}

int main() {
    std::string key;
    print4(&key, 0xd);
    std::cout << key;
}
