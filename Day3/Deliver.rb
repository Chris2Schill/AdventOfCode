class Point
    @@visited = Array.new
    @@delivered = 0
    @@baseX = 0
    @@baseY = 0
    attr_accessor :x, :y, :visited

    def initialize
        @x = @@baseX
        @y = @@baseY
        self.store
    end

    def store
        unless self.visited?
            @@visited.push(self)
            @@delivered += 1
        end
    end

    def visited?
        @@visited.each do |p|
            return true if self.x == p.x and self.y == p.y
        end
        false
    end

    def self.moveUp
        @@baseY += 1
    end
    
    def self.moveDown
        @@baseY -= 1
    end

    def self.moveLeft
        @@baseX -= 1
    end

    def self.moveRight
        @@baseX += 1
    end

    def show
        puts "(#{@x},#{@y})"
    end

    def self.showVisited
        @@visited.each {|p| p.show}
    end

    def self.visited
        @@visited
    end

    def self.delivered
        @@delivered
    end
end

Point.new
File.open('input.txt').each_char do |c|
    if c == '^'
        Point.moveUp
    elsif c == 'v'
        Point.moveDown
    elsif c == '<'
        Point.moveLeft
    elsif c == '>'
        Point.moveRight
    end
    Point.new
end
Point.new
Point.showVisited
p Point.delivered
