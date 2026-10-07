#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    long long input;
    cin >> input;

    while (input != 1) {
        cout << input << " "; 

        if (input % 2  == 0) {
            input /= 2;
        } else {
            input = input * 3 + 1;
        }
    }
    cout << input;
    cout << "\n";
    
}
