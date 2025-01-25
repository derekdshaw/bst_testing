#pragma once

#include "node.h"
#include <utility>

using namespace std;

template<typename NodeValType>
class BTree final {
	
	static_assert(std::is_arithmetic<NodeValType>::value || std::is_same<NodeValType, std::string>::value, "Template parameter NodeValType must be an arithmetic type or std::string");

public:

	BTree() = default;
	~BTree() = default;

	// Remove copy and move. Otherwise
	// the implementation must also copy all nodes in the tree
	// we could just copy the root node, and reassign to the rest of
	// the tree. But that feels unexpected to the caller.
	BTree(const BTree& tree) = delete;
	BTree(const BTree&& tree) = delete;
	BTree& operator=(const BTree& tree) = delete;
	BTree& operator=(const BTree&& tree) = delete;

	// keep the root node as an internal object
	shared_ptr<Node<NodeValType>> GetRoot() noexcept {
		return root;
	}

	// Return the new node
	shared_ptr<Node<NodeValType>> Insert(const NodeValType& nodeValue) noexcept;

	// Delete the node with this value.
	// returns true if the node was deleted, and false if it was not found.
	bool Delete(const NodeValType& nodeValue) noexcept;


	shared_ptr<Node<NodeValType>> Find(const NodeValType& nodeValue) noexcept;

	// output to cout
	void OutputTree() noexcept;

	// output to string
	string OutputTreeString() noexcept;

private:
	shared_ptr<Node<NodeValType>> InsertNode_Internal(const std::shared_ptr<Node<NodeValType>>& root, const NodeValType& nodeValue) noexcept;
	pair<shared_ptr <Node<NodeValType>>, bool> DeleteNode_Internal(const std::shared_ptr<Node<NodeValType>>& root, const NodeValType& nodeValue) noexcept;
	shared_ptr <Node<NodeValType>> minValueNode(const std::shared_ptr <Node<NodeValType>> node) noexcept;
	shared_ptr<Node<NodeValType>> FindNode_Internal(const std::shared_ptr<Node<NodeValType>>& node, const NodeValType& findNodeValue) noexcept;
	void OutputTree_Internal(const shared_ptr<Node<NodeValType>>& node) noexcept;
	void OutputTreeString_Internal(const shared_ptr<Node<NodeValType>>& node, string& output) noexcept;

	shared_ptr<Node<NodeValType>> root;
};

#include "btree.cpp"