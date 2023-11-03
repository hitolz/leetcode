/// 合并两个有序链表
/// https://leetcode.cn/problems/merge-two-sorted-lists/description/

use std::vec;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// pub fn merge_two_lists(
//     list1: Option<Box<ListNode>>,
//     list2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     // 使用尾插法生成新的链表
//     let mut head = ListNode::new(0);
//     let mut tail = &mut head;

//     // let mut l1 = list1;
//     // let mut l2 = list2;
//     let (mut l1, mut l2) = (list1, list2);

//     while let (Some(node1), Some(node2)) = (l1.as_mut(), l2.as_mut()) {
//         if node1.val <= node2.val {
//             let next = node1.next.take();
//             tail.next = l1.take();
//             l1 = next;
//         } else {
//             let next = node2.next.take();
//             tail.next = l2.take();
//             l2 = next;
//         }
//         tail = tail.next.as_mut().unwrap();
//     }
//     tail.next = if l1.is_some() { l1 } else { l2 };
//     head.next
// }

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (list1, list2);

    let mut head = ListNode::new(0);
    let mut node = &mut head;

    while let (Some( node1), Some( node2)) = (l1.as_mut(), l2.as_mut()) {
        if node1.val <= node2.val {
            node.next = Some(node1.to_owned());
            l1 = node1.next.take();
        } else {
            node.next = Some(node2.to_owned());
            l2 = node2.next.take();
        }
        node = node.next.as_mut().unwrap();
    }

    // while l1.is_some() && l2.is_some() {
    //     let (node1, node2) = (l1.as_deref_mut().unwrap(), l2.as_deref_mut().unwrap());
    //     if node1.val < node2.val {
    //         let next = node1.next.take();
    //         node.next = l1.take();
    //         l1 = next;
    //     } else {
    //         let next = node2.next.take();
    //         node.next = l2.take();
    //         l2 = next;
    //     }
    //     node = node.next.as_mut().unwrap();
    // }
    node.next = l1.or(l2);
    head.next
}

fn print_list(list: Option<Box<ListNode>>) {
    let mut current = list;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next;
    }
}

fn main() {
    let lists = merge_two_lists(new_list2(vec![1, 2, 4]), new_list2(vec![1, 3, 4]));
    print_list(lists);
}

// // 头插法
// fn new_list(values: Vec<i32>) -> Option<Box<ListNode>> {
//     // 定义链表 头结点
//     let mut head = None;

//     // 将数组逆序，因为是头插法，输出结果与入参数组里的顺序是相反的，所以先把入参数组逆序。
//     for &val in values.iter().rev() {
//         // 新的节点
//         let mut node = ListNode::new(val);
//         // 新的节点 next 指向 链表头
//         node.next = head;
//         // 链表头指向新的节点
//         head = Some(Box::new(node));
//     }
//     head
// }

// 尾插法
fn new_list2(values: Vec<i32>) -> Option<Box<ListNode>> {
    // 定义两个可变变量，分别叫做 head、tail，表示头和尾，一开始头尾在同一个位置。
    let mut head = None;
    let mut tail = &mut head;

    for i in values {
        let node = ListNode::new(i);
        *tail = Some(Box::new(node));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}
