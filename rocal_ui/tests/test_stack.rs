mod tests {
    use rocal_ui::data_types::stack::Stack;

    #[test]
    fn test_stack_with_primitive_type() {
        let mut stack: Stack<u64> = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.pop(), None);

        stack.push(4);

        assert_eq!(stack.pop(), Some(4));
    }

    #[test]
    fn test_stack_with_obj() {
        let mut stack: Stack<Obj> = Stack::new();

        stack.push(Obj(1));
        stack.push(Obj(2));
        stack.push(Obj(3));

        if let Some(Obj(n)) = stack.pop() {
            assert_eq!(n, 3)
        }

        if let Some(Obj(n)) = stack.pop() {
            assert_eq!(n, 2)
        }

        if let Some(Obj(n)) = stack.pop() {
            assert_eq!(n, 1)
        }

        assert_eq!(stack.pop().is_none(), true);
    }

    #[derive(Clone)]
    struct Obj(u64);
}
