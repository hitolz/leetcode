
// /// 203. 移除链表元素
// ///https://leetcode.cn/problems/remove-linked-list-elements/

// // Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

// pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
//     if head.is_none(){
//         return None;
//     }

//     let mut root = ListNode::new(-1);
//     root.next = head;

//     let mut prev_node = &mut root;
//     let mut next_node = &root.next;

//     while let Some(mut next) = next_node.as_mut().unwrap().next {
//         if next.val == val{
//             let tmp = next.next;
//             prev_node.next = tmp.clone();
//             next.as_mut().next = None;
//             next = tmp.unwrap();
//         }else{
//             prev_node = &mut (*next_node.as_mut().unwrap().to_owned());
//             next_node = &next_node.as_mut().unwrap().next;
//         }
//     }
//     if next_node.is_none() {
//         prev_node.next = None;
//     }
//     root.next
// }

// pub fn run() {
//     let vec = vec![1, 2, 6, 3, 4, 5, 6];
//     let head = new_list(vec);
//     let val = 6;
//     let head = remove_elements(head, val);
//     print_list(head);
// }

// fn new_list(values: Vec<i32>) -> Option<Box<ListNode>> {
//     // 定义两个可变变量，分别叫做 head、tail，表示头和尾，一开始头尾在同一个位置。
//     let mut head = None;
//     let mut tail = &mut head;

//     for i in values {
//         let node = ListNode::new(i);
//         *tail = Some(Box::new(node));
//         tail = &mut tail.as_mut().unwrap().next;
//     }
//     head
// }

// fn print_list(list: Option<Box<ListNode>>) {
//     let mut current = list;
//     while let Some(node) = current {
//         print!("{},", node.val);
//         current = node.next;
//     }
// }
