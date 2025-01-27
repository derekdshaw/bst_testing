#include "bst_test.h"

bool g_ShowOutput{ false };

// just run the tests.
int main(int argc, char* argv[])
{
    return lest::run(specification, argc, argv);
}
