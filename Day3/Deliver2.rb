class Point
    @@visited = Array.new
    @@delivered = 0
    @@santaBaseX = 0
    @@santaBaseY = 0
    @@roboSantaBaseX = 0
    @@roboSantaBaseY = 0
    attr_accessor :x, :y, :visited

    def initialize bool
        if bool
            @x = @@santaBaseX
            @y = @@santaBaseY
            self.store
        else
            @x = @@roboSantaBaseX
            @y = @@roboSantaBaseY
            self.store
        end
    end

    private
    def store
        unless self.visited?
            @@visited.push(self)
            @@delivered += 1
        end
    end

    private
    def visited?
        @@visited.each do |p|
            return true if self.x == p.x and self.y == p.y
        end
        false
    end

    
    def self.moveUp
        @@santaBaseY += 1
    end
    def self.rMoveUp
        @@roboSantaBaseY += 1
    end
    
    def self.moveDown
        @@santaBaseY -= 1
    end
    def self.rMoveDown
        @@roboSantaBaseY -= 1
    end

    def self.moveLeft
        @@santaBaseX -= 1
    end
    def self.rMoveLeft
        @@roboSantaBaseX -= 1
    end

    def self.moveRight
        @@santaBaseX += 1
    end
    def self.rMoveRight
        @@roboSantaBaseX += 1
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

Point.new true
santasTurn = true
File.open('input.txt').each_char do |c|
    if santasTurn
        if c == '^'
            Point.moveUp 
        elsif c == 'v'
            Point.moveDown
        elsif c == '<'
            Point.moveLeft
        elsif c == '>'
            Point.moveRight
        end
        santasTurn = false
    else
        if c == '^'
            Point.rMoveUp
        elsif c == 'v'
            Point.rMoveDown
        elsif c == '<'
            Point.rMoveLeft
        elsif c == '>'
            Point.rMoveRight
        end
        santasTurn = true
    end
    Point.new santasTurn
end
Point.showVisited
p Point.delivered
