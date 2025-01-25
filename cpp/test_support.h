#pragma once

#include "btree.h"
#include <unordered_set>

void SetNodeShowDeleteOutput(bool doOutput) noexcept;

template<typename NodeValType>
BTree<NodeValType>* BuildTreeForTest() {
    auto tree = new BTree<NodeValType>();
    auto root = tree->Insert(5);

    auto node = tree->Insert(11);
    node = tree->Insert(8);
    node = tree->Insert(9);
    node = tree->Insert(15);
    node = tree->Insert(2);

    return tree;
}

unordered_set<int> GetLargeData() noexcept;