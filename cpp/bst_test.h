#include "lest/lest.hpp"

#include "bst.h"
#include "test_support.h"

#include <unordered_set>
#include <chrono>

using namespace std;

const lest::test specification[] =
{
    CASE("Root node inserted")
    {
        auto tree = new BST<int>();
        auto root = tree->Insert(5);

        EXPECT(root == tree->GetRoot());
        EXPECT(tree->GetRoot()->value == 5);

        auto output_tree = tree->OutputTreeString();

        EXPECT(output_tree == "5 ");

        delete tree;
    },

    CASE("Insert multiple nodes, out of order")
    {
        auto tree { BuildTreeForTest<int>() };

        auto result = tree->OutputTreeString();
        EXPECT(result == "2 5 8 9 11 15 ");

        delete tree;
    },
    CASE("Find node")
    {
        auto tree { BuildTreeForTest<int>() };

        auto foundNode = tree->Find(9);
        EXPECT(foundNode);
        EXPECT(foundNode->value == 9);

        delete tree;
    },
    CASE("Find, node missing")
    {
        auto tree { BuildTreeForTest<int>() };

        auto foundNode = tree->Find(12);
        EXPECT(!foundNode);

        delete tree;
    },
    CASE("Delete single node")
    {
        auto tree { BuildTreeForTest<int>() };

        auto didDelete = tree->Delete(9);
        EXPECT(didDelete);

        auto result = tree->OutputTreeString();
        EXPECT(result == "2 5 8 11 15 ");

        delete tree;
    },

    CASE("Delete, node missing")
    {
        auto tree { BuildTreeForTest<int>() };

        auto didDelete = tree->Delete(12);
        EXPECT(!didDelete);

        delete tree;
    },

    CASE("Delete root")
    {
        auto tree { BuildTreeForTest<int>() };

        auto didDelete = tree->Delete(5);
        EXPECT(didDelete);

        auto result = tree->OutputTreeString();
        EXPECT(result == "2 8 9 11 15 ");

        delete tree;
    },

    CASE("Build large tree")
    {
        // data set of ints in no specific order
        const auto uset { GetLargeData() };

        auto tree = new BST<int>();
        auto root = tree->Insert(0);

        auto start = std::chrono::high_resolution_clock::now();

        for (auto itr = uset.begin(); itr != uset.end(); itr++) {
            tree->Insert(*itr);
        }

        // Get the end time 
        auto end = std::chrono::high_resolution_clock::now();
        
        // Calculate the duration 
        std::chrono::duration<double, std::milli> duration = end - start;
        
        // Output the duration in seconds 
        std::cout << "Time taken by large set (" << uset.size() << " items) insert: " << duration.count() << " milliseconds" << std::endl;

        SetNodeShowDeleteOutput(false);

        delete tree;
    },

    CASE("Delete from large tree")
    {
        // data set of ints in no specific order
        const auto uset { GetLargeData() };

        auto tree = new BST<int>();
        auto root = tree->Insert(0);

        int i = 0;
        int deleteVal{ -1 };
        for (auto itr = uset.begin(); itr != uset.end(); itr++) {
            tree->Insert(*itr);
            if (i == 500000) {
                deleteVal = *itr;
            }
            i++;
        }

        auto start = std::chrono::high_resolution_clock::now();

        tree->Delete(deleteVal);

        // Get the end time 
        auto end = std::chrono::high_resolution_clock::now();

        // Calculate the duration
        std::chrono::duration<double, std::milli> duration = end - start;

        // Output the duration in seconds 
        std::cout << "Time taken by large set (" << uset.size() << " items) delete one: " << duration.count() << " milliseconds" << std::endl;

        SetNodeShowDeleteOutput(false);

        delete tree;
    },
};