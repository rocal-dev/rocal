mod tests {
    use rocal_ui::data_types::queue::Queue;

    #[test]
    fn test_queue_with_primitive_type() {
        let mut queue: Queue<u64> = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));

        queue.enqueue(4);

        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(5);
        assert_eq!(queue.dequeue(), Some(5));

        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_queue_with_obj() {
        let mut queue: Queue<Obj> = Queue::new();

        queue.enqueue(Obj { id: 1 });
        queue.enqueue(Obj { id: 2 });
        queue.enqueue(Obj { id: 3 });

        let obj1 = queue.dequeue();
        let obj2 = queue.dequeue();
        let obj3 = queue.dequeue();
        let obj4 = queue.dequeue();

        assert!(obj1.is_some());
        assert!(obj2.is_some());
        assert!(obj3.is_some());
        assert_ne!(obj4.is_some(), true);
    }

    #[derive(Clone)]
    struct Obj {
        #[allow(dead_code)]
        id: u32,
    }
}
