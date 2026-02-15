class Node<T> {
  value: T;                      // 该节点的数据
  next: Node<T> | null = null;   // 下一个节点，如果该节点是最后一个，则为 null
  prev: Node<T> | null = null;   // 上一个节点，如果是第一个，则为 null
  
  constructor(value: T) { 
    this.value = value;  // 把传进来的值存到 value 里，另外两个属性默认为 null
  }
}

export class LinkedList<T> {
  private head: Node<T> | null = null;  // 链表头
  private tail: Node<T> | null = null;  // 链表尾
  
  public push(element: T): void {
    const newNode = new Node(element); // 创建新节点，先把值传进来
  
    if (this.tail === null) {
      this.head = newNode;
      this.tail = newNode;
      // 如果链表本身是空的，那么链表头和链表尾都是这个节点
    } else {
      this.tail.next = newNode; // 其他情况就是说链表本身不是空的，让当前尾节点的 next 指针指向新节点
      newNode.prev = this.tail; // 而新节点的前一个位置就是链表尾
      this.tail = newNode; // 新节点成为链表的最后一个节点
    }

  }

  public pop(): T {
    if (this.tail === null) {
      throw new Error('List is empty'); // 空链表做不了 pop() 这个操作，予以报错
    }
    
    const returnValue = this.tail.value; // 最后一个节点的值，pop() 正常执行的结果
    
    if (this.head === this.tail) {
      this.head = null;
      this.tail = null;
      // 只有一个节点的情况下，执行完链表变空
    } else {
      this.tail = this.tail.prev; // 执行完链表尾迁移
      this.tail!.next = null; // 链表尾的 next 必须为空
    }

    return returnValue; 
  }
  
  public unshift(element: T): void {
    // 在链表头添加元素，原理基本同 push() ，位置不同方法相反
    const newNode = new Node(element);
    
    if (this.head === null) {
      this.head = newNode;
      this.tail = newNode;
    } else {
      newNode.next = this.head;
      this.head.prev = newNode;
      this.head = newNode;
    }
  }
  
  public shift(): T {
    // 移除并返回链表开头的元素，原理同 pop()，位置不同方法相反
    if (this.head === null) {
      throw new Error('List is empty');
    }
    
    const returnValue = this.head.value;
    
    if (this.head === this.tail) {
      this.head = null;
      this.tail = null;
    } else {
      this.head = this.head.next;
      this.head!.prev = null; // 链表头的 prev 必须为空
    }
    
    return returnValue;
  }
  
  public delete(element: T): void {

    let currentNode = this.head; // 从链表头开始
    
    while (currentNode !== null) { 
      
      if (currentNode.value === element) {
        // 如果当前节点的值直接等于目标值，就直接删除它
        
        if (this.head === this.tail) {
          // 链表变空，只有一个节点的情况，删除后链表为空
          this.head = null;
          this.tail = null;
        }
        
        else if (currentNode === this.head) {
          // 删除第一个节点，后边还有节点
          this.head = currentNode.next;  // 链表头从下一个节点开始
          this.head!.prev = null; 
        }
        
        else if (currentNode === this.tail) {
          // 删除最后一个节点，前边还有节点
          this.tail = currentNode.prev;
          this.tail!.next = null; 
        }
        
        else { // 当前要删除的节点前后都有节点
          currentNode.prev!.next = currentNode.next;
          currentNode.next!.prev = currentNode.prev;
          // 去掉当前节点后，当前节点的前后节点重新建立连接
        }
        
        return; // 退出函数
      }
      
      // 第5步：没找到，继续往后找下一个节点
      currentNode = currentNode.next;
    }

  }
  
  public count(): number {

    let count = 0;
    
    let currentNode = this.head; // 从链表头开始
    
    while (currentNode !== null) {
      count = count + 1;        
      currentNode = currentNode.next;  // 只要当前节点不是空的，当前节点后移直到为空时停止，因为从0开始算，所以几步意味着链表有几个节点
    }
    
    return count;
  }
}