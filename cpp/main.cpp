#include "btree_test.h"

bool g_ShowOutput{ true };

// just run the tests.
int main(int argc, char* argv[])
{
    return lest::run(specification, argc, argv);
}
