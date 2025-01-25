
// To keep the code cleaner, break the declaration from the impl.

template<typename NodeValType>
shared_ptr<Node<NodeValType>> BTree<NodeValType>::Insert(const NodeValType& nodeValue) noexcept {

	if (!root) {
		root = std::make_shared <Node<NodeValType>>(nodeValue);
		return root;
	}

	return InsertNode_Internal(root, nodeValue);
}

template<typename NodeValType>
bool BTree<NodeValType>::Delete(const NodeValType& nodeValue) noexcept {
	const auto& [updatedNode, didDelete]{ DeleteNode_Internal(root, nodeValue) };

	// if we deleted the root update the root node.
	if (!root) {
		root = updatedNode;
	}

	return didDelete;
}

template<typename NodeValType>
shared_ptr<Node<NodeValType>> BTree<NodeValType>::Find(const NodeValType& findNodeValue) noexcept {
	return FindNode_Internal(root, findNodeValue);
}

template<typename NodeValType>
void BTree<NodeValType>::OutputTree() noexcept {
	OutputTree_Internal(root);
	cout << endl;
}

template<typename NodeValType>
string BTree<NodeValType>::OutputTreeString() noexcept {

	string result;
	OutputTreeString_Internal(root, result);
	return result;
}

template<typename NodeValType>
std::shared_ptr<Node<NodeValType>> BTree<NodeValType>::InsertNode_Internal(const std::shared_ptr<Node<NodeValType>>& node, const NodeValType& nodeValue) noexcept {

	if (!node) {
		return make_shared<Node<NodeValType>>(nodeValue);
	}

	// Recurse the tree to insert
	if (nodeValue < node->value) {
		node->left = InsertNode_Internal(node->left, nodeValue);
	}
	else if (nodeValue > node->value) {
		node->right = InsertNode_Internal(node->right, nodeValue);
	}

	return node;
}

template<typename NodeValType>
pair<shared_ptr <Node<NodeValType>>, bool> BTree<NodeValType>::DeleteNode_Internal(const std::shared_ptr<Node<NodeValType>>& node, const NodeValType& nodeValue) noexcept {

	if (!node) {
		return { node, false };
	}

	// recurse to find the node
	if (nodeValue < node->value) {
		const auto& [updatedNode, result] = DeleteNode_Internal(node->left, nodeValue);
		node->left = updatedNode;
		if (!result) return { node, result };
	}
	else if(nodeValue > node->value) {
		const auto& [updatedNode, result] = DeleteNode_Internal(node->right, nodeValue);
		node->right = updatedNode;
		if (!result) return { node, result };
	}
	else {
		// node value found

		// either one child or no children
		// we do not delete the node but instead let it go out of scope
		// to be destructed
		if (!node->left) {
			return { node->right, true };
		}
		else if (!node->right) {
			return { node->left, true };
		}

		// a node with two children, find the inorder successor. The
		// smallest value in the right subtree
		auto temp = minValueNode(node->right);
		node->value = temp->value;
		const auto& [updatedNode, result] = DeleteNode_Internal(node->right, temp->value);
		node->right = updatedNode;
	}

	return { node, true };
}

template<typename NodeValType>
std::shared_ptr <Node<NodeValType>> BTree<NodeValType>::minValueNode(const std::shared_ptr <Node<NodeValType>> node) noexcept {
	auto current = node;
	while (current && current->left) {
		current = current->left;
	}

	return current;
}

template<typename NodeValType>
shared_ptr<Node<NodeValType>> BTree<NodeValType>::FindNode_Internal(const std::shared_ptr<Node<NodeValType>>& node, const NodeValType& findNodeValue) noexcept {

	if (!node || node->value == findNodeValue)
		return node;
	
	if (findNodeValue < node->value) {
		return FindNode_Internal(node->left, findNodeValue);
	}

	return FindNode_Internal(node->right, findNodeValue);
}

template<typename NodeValType>
void BTree<NodeValType>::OutputTree_Internal(const shared_ptr<Node<NodeValType>>& node) noexcept {

	// output using in order traversal.
	if (node) {
		OutputTree_Internal(node->left);
		cout << node->value << " ";
		OutputTree_Internal(node->right);
	}
}

template<typename NodeValType>
void BTree<NodeValType>::OutputTreeString_Internal(const shared_ptr<Node<NodeValType>>& node, string& output) noexcept {
	
	if (node) {
		OutputTreeString_Internal(node->left, output);
		if constexpr (std::is_arithmetic<NodeValType>::value) {
			output.append(to_string(node->value));
			output.append(" ");
		}
		else if constexpr (is_same<NodeValType, bool>::value) {
			output.append(node->value ? "true" : "false");
		}
		else if constexpr (is_same<NodeValType, string>::value) {
			output.append(node->value);
		}
		OutputTreeString_Internal(node->right, output);
	}
}