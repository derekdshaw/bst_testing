package btree

import (
	"fmt"
	"math/rand"
	"testing"
	"time"
)

func BuildTreeForTest(values []int) *BST[int] {
	bst := NewBST[int](func(a, b int) bool { return a < b })

	for _, v := range values {
		bst.Insert(v)
	}

	return bst
}

func BuildLargeDataForTest() []int {
	values := make([]int, 0, 1000000)
	for i := 0; i < 1000000; i++ {
		values = append(values, rand.Intn(2000000)+1)
	}

	return values
}

func TestBST(t *testing.T) {
	// Create a BST for integers
	values := []int{5, 11, 8, 9, 15, 2}
	bst := BuildTreeForTest(values)

	// Test Find
	for _, v := range values {
		if !bst.Find(v) {
			t.Errorf("Find(%d) = false, want true", v)
		}
	}

	if bst.Find(100) {
		t.Error("Find(100) = true, want false")
	}

	// Test InorderTraversal
	inorder := bst.InorderTraversal()
	expected := []int{2, 5, 8, 9, 11, 15}
	for i, v := range inorder {
		if v != expected[i] {
			t.Errorf("InorderTraversal()[%d] = %d, want %d", i, v, expected[i])
		}
	}

	// Test insert duplicate
	bst.Insert(5)
	treeString := bst.String()
	if treeString != "2 5 8 9 11 15 " {
		t.Errorf("After Insert(5) duplicate, treeString = %s, want \"2 5 8 9 11 15 \"", treeString)
	}

	// Test Delete
	bst.Delete(8)
	if bst.Find(8) {
		t.Error("After Delete(8), Find(8) = true, want false")
	}

	// Test deleting root
	bst.Delete(5)
	if bst.Find(5) {
		t.Error("After Delete(5), Find(5) = true, want false")
	}

	// Verify remaining values
	remainingValues := []int{2, 9, 11, 15}
	for _, v := range remainingValues {
		if !bst.Find(v) {
			t.Errorf("After deletions, Find(%d) = false, want true", v)
		}
	}

	// Test string BST
	strBST := NewBST[string](func(a, b string) bool { return a < b })
	strings := []string{"banana", "apple", "cherry"}
	for _, s := range strings {
		strBST.Insert(s)
	}

	inorderStrings := strBST.InorderTraversal()
	expectedStrings := []string{"apple", "banana", "cherry"}
	for i, s := range inorderStrings {
		if s != expectedStrings[i] {
			t.Errorf("String BST InorderTraversal()[%d] = %s, want %s", i, s, expectedStrings[i])
		}
	}
}

func TestBuildLargeBST(t *testing.T) {
	values := BuildLargeDataForTest()

	start := time.Now()
	BuildTreeForTest(values)
	duration := time.Since(start)

	fmt.Printf("Time to build tree with %d nodes: %v\n", len(values), duration)
}

func BenchmarkBuildLargeBST(b *testing.B) {
	values := BuildLargeDataForTest()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		BuildTreeForTest(values)
	}
}

func BenchmarkDeleteFromLargeBST(b *testing.B) {
	values := BuildLargeDataForTest()
	valueToDelete := values[500000]

	// Build the initial tree before starting the benchmark
	bst := BuildTreeForTest(values)

	// Reset timer before starting iterations
	b.ResetTimer()

	// Run the delete operation b.N times
	for i := 0; i < b.N; i++ {
		// Only measure the deletion operation
		bst.Delete(valueToDelete)

		// NOTE: I was trying to stop/start the timer around the insertion,
		// so that only the delete would be measured. However, doing this
		// causes the benchmark tests to hang for some reason.

		// Stop timer for reinsertion
		// b.StopTimer()
		bst.Insert(valueToDelete)
		// b.StartTimer()
	}
}

func TestDeleteFromLargeBST(t *testing.T) {
	values := BuildLargeDataForTest()
	bst := BuildTreeForTest(values)
	valueToDelete := values[500000]

	if !bst.Find(valueToDelete) {
		t.Fatalf("Test setup failed: value %d not found in tree before deletion", valueToDelete)
	}

	start := time.Now()
	bst.Delete(valueToDelete)
	duration := time.Since(start)

	fmt.Printf("Time to delete value %d from tree with %d nodes: %d nanoseconds\n", valueToDelete, len(values), duration.Nanoseconds())

	if bst.Find(valueToDelete) {
		t.Errorf("Value %d still found in tree after deletion", valueToDelete)
	}
}
