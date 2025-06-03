#include "testlib.h"

using namespace std;

int main(int argc, char *argv[]) {
    registerGen(argc, argv, 1);

    int a = rnd.next(1, 1'000'000);
    int b = rnd.next(1, 1'000'000);
    
    println(a, b);
    
    return 0;
}
