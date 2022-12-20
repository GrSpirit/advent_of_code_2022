#include <iostream>
#include <vector>
#include <list>

void mix(std::list<int64_t>& numbers, std::vector<std::list<int64_t>::iterator>& positions) {
    int64_t n = numbers.size();
    for (auto& it: positions) {
        int64_t x = *it;
        int64_t pos = std::distance(numbers.begin(), it);
        numbers.erase(it);
        int64_t new_pos = (pos + x) % (n - 1);
        if (new_pos < 0) new_pos += (n - 1);
        auto insert_pos = std::next(numbers.begin(), new_pos);
        it = numbers.insert(insert_pos, x);
    }
}

void print_answer(const std::list<int64_t>& numbers) {
    int n = numbers.size();
    auto it0 = std::find(numbers.begin(), numbers.end(), 0);
    auto pos0 = std::distance(numbers.begin(), it0);
    std::cout << *std::next(numbers.begin(), (pos0 + 1000) % n) << " " << *std::next(numbers.begin(), (pos0 + 2000) % n) << " " << *std::next(numbers.begin(), (pos0 + 3000) % n) << "\n";
    std::cout << (*std::next(numbers.begin(), (pos0 + 1000) % n) + *std::next(numbers.begin(), (pos0 + 2000) % n) + *std::next(numbers.begin(), (pos0 + 3000) % n)) << "\n";
}

int main() {
    std::list<int64_t> numbers;
    int64_t k = 811589153;
    int64_t x;
    while (std::cin >> x) {
        numbers.push_back(x * k);
    }
    int n = numbers.size();
    std::vector<std::list<int64_t>::iterator> positions;
    positions.reserve(n);
    for (auto it = numbers.begin(); it != numbers.end(); ++it) {
        positions.push_back(it);
    }
    for (int i = 0; i < 10; i++) {
        mix(numbers, positions);
    }
    print_answer(numbers);
}
