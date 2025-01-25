package btree

import (
	"fmt"
	"strings"
)

// Node represents a node in the binary search tree
type BSTNode[T comparable] struct {
	Value T
	Left  *BSTNode[T]
	Right *BSTNode[T]
}

// BST represents the binary search tree
type BST[T comparable] struct {
	root *BSTNode[T]
	less func(a, b T) bool // Custom comparator
}

// NewBST creates a new binary search tree with a custom comparator
func NewBST[T comparable](less func(a, b T) bool) *BST[T] {
	return &BST[T]{
		root: nil,
		less: less,
	}
}

// Insert adds a new value to the BST
func (t *BST[T]) Insert(value T) {
	t.root = t.insert(t.root, value)
}

func (t *BST[T]) insert(node *BSTNode[T], value T) *BSTNode[T] {
	if node == nil {
		return &BSTNode[T]{Value: value}
	}

	if t.less(value, node.Value) {
		node.Left = t.insert(node.Left, value)
	} else if t.less(node.Value, value) {
		node.Right = t.insert(node.Right, value)
	}

	return node
}

// Find searches for a value in the BST
func (t *BST[T]) Find(value T) bool {
	return t.find(t.root, value)
}

func (t *BST[T]) find(node *BSTNode[T], value T) bool {
	if node == nil {
		return false
	}

	if value == node.Value {
		return true
	}

	if t.less(value, node.Value) {
		return t.find(node.Left, value)
	}
	return t.find(node.Right, value)
}

// Delete removes a value from the BST
func (t *BST[T]) Delete(value T) {
	t.root = t.delete(t.root, value)
}

func (t *BST[T]) delete(node *BSTNode[T], value T) *BSTNode[T] {
	if node == nil {
		return nil
	}

	if t.less(value, node.Value) {
		node.Left = t.delete(node.Left, value)
	} else if t.less(node.Value, value) {
		node.Right = t.delete(node.Right, value)
	} else {
		// Node to delete found

		// Case 1: Leaf node
		if node.Left == nil && node.Right == nil {
			return nil
		}

		// Case 2: Node with only one child
		if node.Left == nil {
			return node.Right
		}
		if node.Right == nil {
			return node.Left
		}

		// Case 3: Node with two children
		// Find the smallest value in the right subtree (successor)
		minNode := t.findMinNode(node.Right)
		node.Value = minNode.Value
		// Delete the successor
		node.Right = t.delete(node.Right, minNode.Value)
	}

	return node
}

// findMinNode finds the node with minimum value in the subtree
func (t *BST[T]) findMinNode(node *BSTNode[T]) *BSTNode[T] {
	current := node
	for current.Left != nil {
		current = current.Left
	}
	return current
}

// InorderTraversal returns the inorder traversal of the BST
func (t *BST[T]) InorderTraversal() []T {
	var result []T
	t.inorder(t.root, &result)
	return result
}

func (t *BST[T]) inorder(node *BSTNode[T], result *[]T) {
	if node != nil {
		t.inorder(node.Left, result)
		*result = append(*result, node.Value)
		t.inorder(node.Right, result)
	}
}

// String returns a string representation of the BST
func (t *BST[T]) String() string {
	var sb strings.Builder
	t.printTree(t.root, 0, &sb)
	return sb.String()
}

func (t *BST[T]) printTree(node *BSTNode[T], level int, sb *strings.Builder) {
	if node != nil {
		t.printTree(node.Left, level+1, sb)
		//sb.WriteString(strings.Repeat("    ", level))
		sb.WriteString(fmt.Sprintf("%v ", node.Value))
		t.printTree(node.Right, level+1, sb)
	}
}
