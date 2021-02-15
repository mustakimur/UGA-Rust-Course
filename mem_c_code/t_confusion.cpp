#include <iostream>

using namespace std;

int main()
{
    short a = 10;
    int *b = (int *)&a;

    short *c = &a;

    cout << a << " " << *b << " " << *c << endl;
}