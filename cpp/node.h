#pragma once

#include <iostream>
#include <memory>

extern bool g_ShowOutput;

template<typename ValType>
struct Node {
	Node() noexcept = default;
	Node(const ValType& val) noexcept : value{ val } {};
	~Node() noexcept {
		
		if (g_ShowOutput) {
			// In order to validate the nodes go out of scope as expected.
			std::cout << "Node with value: " << value << " deleted." << std::endl;
		}
	}

	ValType value;
	std::shared_ptr<Node> left;
	std::shared_ptr<Node> right;
};