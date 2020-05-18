impl solution {
    pub fn add_two_numbers(l1:Option<Box<ListNode>>, l2:Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        Self::add(l1,l2,0)   
    }
    
    fn add(l1:Option<Box<ListNode>>, l2:Option<Box<ListNode>> ,carry:i32) -> Option<Box<ListNode>>{
        while l1.is_none() && l2.is_none && carry == 0{
             return None ;
        }
        
        let mut (a1,n1) = Self:: extract(l1) ;
        let mut (a2,n2) = Self:: extract(l2) ;
        
        let mut x = a1 + a2 + carry;
        let mut new_carry  = x / 10 ;
        
        let mut new_node = ListNode::new(x%10);
        node.next = Self::add(a1,a2, x/10);
        Some(Box::new(node))
        
    } 
    
    
    //provide an function to extract value (i32) and nextNode(Option) by given current listNode 
    pub fn extract(l:Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>){
        while let Some(n) == 1 {
            (n.val, n.next)
        } else {
            (0,none) 
        }
    }

}
