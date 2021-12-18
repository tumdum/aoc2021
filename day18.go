package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type Node struct {
	Left, Rigth *Node
	val         int64
}

func (n *Node) Clone() *Node {
	if n.Left == nil && n.Rigth == nil {
		return &Node{nil, nil, n.val}
	}
	return &Node{n.Left.Clone(), n.Rigth.Clone(), 0}
}

func (n *Node) String() string {
	if n.Left != nil && n.Rigth != nil {
		return fmt.Sprintf("[%v,%v]", n.Left, n.Rigth)
	}
	return fmt.Sprintf("%v", n.val)
}

func parse(s string) (*Node, string) {
	if s[0] == '[' {
		l, rest := parse(s[1:])
		if rest[0] != ',' {
			panic("inalid1")
		}
		r, rest := parse(rest[1:])
		if rest[0] != ']' {
			panic("inalid2")
		}
		return &Node{l, r, 0}, rest[1:]
	} else {
		start := 0
		end := 0
		for end < len(s) && s[end] >= '0' && s[end] <= '9' {
			end++
		}
		n, e := strconv.Atoi(s[start:end])
		check(e)
		return &Node{nil, nil, int64(n)}, s[end:]
	}
	panic("unreachable")
}

func MustParse(s string) *Node {
	n, _ := parse(s)
	return n
}

func nested(n *Node, d int, parent *Node) (bool, *int64, *int64) {
	if n.Left == nil && n.Rigth == nil {
		return false, nil, nil
	}
	if d == 4 {
		if parent.Left == n {
			parent.Left = &Node{nil, nil, 0}
		} else {
			parent.Rigth = &Node{nil, nil, 0}
		}
		return true, &n.Left.val, &n.Rigth.val
	}

	if found, l, r := nested(n.Left, d+1, n); found {
		if r != nil {
			addToLeft(n.Rigth, *r)
			r = nil
		}
		return true, l, r
	}
	if found, l, r := nested(n.Rigth, d+1, n); found {
		if l != nil {
			addToRight(n.Left, *l)
			l = nil
		}
		return true, l, r
	}
	return false, nil, nil
}

func addToLeft(n *Node, v int64) {
	if n.Left == nil && n.Rigth == nil {
		n.val += v
		return
	}
	if n.Left != nil {
		addToLeft(n.Left, v)
		return
	}
	addToLeft(n.Rigth, v)
}

func addToRight(n *Node, v int64) {
	if n.Left == nil && n.Rigth == nil {
		n.val += v
		return
	}
	if n.Rigth != nil {
		addToRight(n.Rigth, v)
		return
	}
	addToRight(n.Left, v)
}

func validate(input string, expected string) {
	n, _ := parse(input)
	did, _, _ := nested(n, 0, nil)
	if !did {
		panic("didn't modify")

	}
	got := fmt.Sprintf("%v", n)
	if got != expected {
		fmt.Println("expected '%s'", expected)
		fmt.Println("     got '%s'", got)
		panic("FAILED")
	}
}

func validate_sum(input []string, expected string) {
	nums := []*Node{}
	for _, n := range input {
		nums = append(nums, MustParse(n))
	}
	n := sum(nums)
	got := fmt.Sprintf("%v", n)
	if got != expected {
		fmt.Printf("expected '%s'\n", expected)
		fmt.Printf("     got '%s'\n", got)
		panic("FAILED")
	}
}

func split(n *Node) bool {
	if n.Left == nil && n.Rigth == nil {
		if n.val >= 10 {
			l := int64(math.Floor(float64(n.val) / 2))
			r := int64(math.Ceil(float64(n.val) / 2))
			n.val = 0
			n.Left = &Node{nil, nil, l}
			n.Rigth = &Node{nil, nil, r}
			return true
		}
		return false
	}
	if did := split(n.Left); did {
		return true
	}
	return split(n.Rigth)
}

func add(l, r *Node) *Node {
	ret := &Node{l.Clone(), r.Clone(), 0}
	for {
		if didNested, _, _ := nested(ret, 0, nil); didNested {
			//	fmt.Println("after explode", ret)
			continue
		}
		if didSplit := split(ret); didSplit {
			//	fmt.Println("after split", ret)
			continue
		}
		break
	}
	return ret
}

func sum(nums []*Node) *Node {
	cur := nums[0].Clone()
	for i := 1; i < len(nums); i++ {
		cur = add(cur, nums[i])
	}
	return cur
}

func magnitude(n *Node) int64 {
	if n.Left == nil && n.Rigth == nil {
		return n.val
	}
	return 3*magnitude(n.Left) + 2*magnitude(n.Rigth)
}

func main() {
	validate("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")
	validate("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]")
	validate("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]")
	validate("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
	validate("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")

	// fmt.Println(add(MustParse("[[[[4,3],4],4],[7,[[8,4],9]]]"), MustParse("[1,1]")))

	validate_sum([]string{"[1,1]", "[2,2]", "[3,3]", "[4,4]"}, "[[[[1,1],[2,2]],[3,3]],[4,4]]")
	validate_sum([]string{"[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"}, "[[[[3,0],[5,3]],[4,4]],[5,5]]")
	validate_sum([]string{"[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"}, "[[[[5,0],[7,4]],[5,5]],[6,6]]")

	validate_sum([]string{"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]", "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]", "[7,[5,[[3,8],[1,4]]]]", "[[2,[2,2]],[8,[8,1]]]", "[2,9]", "[1,[[[9,3],9],[[9,0],[0,7]]]]", "[[[5,[7,4]],7],1]", "[[[[4,2],2],6],[8,7]]"}, "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")

	// fmt.Println(magnitude(MustParse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")))

	numbers := []*Node{}
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		numbers = append(numbers, MustParse(scanner.Text()))
	}

	fmt.Println(magnitude(sum(numbers)))

	var best int64
	for i := 0; i != len(numbers); i++ {
		for j := 0; j != len(numbers); j++ {
			if i == j {
				continue
			}
			ij := magnitude(add(numbers[i], numbers[j]))
			if ij > best {
				best = ij
			}
			ji := magnitude(add(numbers[j], numbers[i]))
			if ji > best {
				best = ji
			}
		}
	}
	fmt.Println(best)
}
