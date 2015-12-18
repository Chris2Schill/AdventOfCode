class Node
    attr_accessor :value, :left, :right
    def initialize x
        @value = x
        @left = nil
        @right = nil
    end
    def set_left x
        @left = Node.new(x) 
    end
    def set_right x
        @right = Node.new(x) 
    end
end

def insert head, x
    if x < head.value
        insert(head.left, x) if head.left != nil
        head.set_left(x) if head.left == nil
    elsif x >= head.value
        insert(head.right, x) if head.right != nil
        head.set_right(x) if head.right == nil
    end
end

def delete x
end

def search x
end

def in_order head
    if head.left != nil
        in_order head.left
    end
    print "#{head.value} "
    if head.right != nil
        in_order head.right
    end
end

tree = Node.new(5)
in_order(tree); puts
insert(tree,4)
in_order(tree)

