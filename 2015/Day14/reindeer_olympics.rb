class Reindeer
    attr_accessor :distance

    def initialize name, velocity, endurance, sleep
        @name = name 
        @velocity = velocity
        @endurance = endurance
        @sleep = sleep
        @stamina = endurance
        @distance = 0
        @fatigue = 0  
    end

    def take_turn
        if @stamina > 0
            keep_flying()
            go_to_sleep if @stamina <= 0
        elsif @fatigue > 0
            rest
        end

        wake_up if @stamina <= 0 and @fatigue <= 0

    end
    
    private
    def keep_flying
        @distance += @velocity 
        @stamina -= 1
    end

    private
    def go_to_sleep
        @fatigue = @sleep
    end

    private
    def rest
        @fatigue -= 1
    end

    private
    def wake_up
        @stamina = @endurance
    end
end

def getReindeers
    reindeers = Array.new
    File.readlines('Day14/input.txt').each do |line|
        name = line.split(' ').first

        attributes_match = line.scan(/[0-9]+/)
        velocity = attributes_match[0].to_i
        endurance = attributes_match[1].to_i
        sleep = attributes_match[2].to_i

        reindeers << Reindeer.new(name, velocity, endurance, sleep) 
    end
    reindeers
end

reindeers = getReindeers()
maxDistance = 0
2503.times do |second|
#    puts "------Second: #{second+1}------"
    reindeers.each do |reindeer|
        reindeer.take_turn
        if reindeer.distance > maxDistance
            maxDistance = reindeer.distance
        end
#        puts "#{reindeer.name} traveled #{reindeer.distance}"
    end
#    puts
end

puts "Max Distance: #{maxDistance}"
