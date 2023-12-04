class Reindeer
    attr_accessor :name, :velocity, :endurance, :sleep, :stamina, :distance, :alarm, :points
    def initialize name, velocity, endurance, sleep
        @name = name 
        @velocity = velocity
        @endurance = endurance
        @sleep = sleep
        @stamina = endurance
        @distance = 0
        @alarm = 0  
        @points = 0
    end

    def print
        puts "#{@name} #{@velocity} #{@endurance} #{@sleep} #{@stamina} #{@distance} #{@alarm}"
    end
end

reindeers = Array.new

File.readlines('input.txt').each do |line|
    name = line.split(' ').first
    attributes_match = line.scan(/[0-9]+/)
    velocity = attributes_match[0].to_i
    endurance = attributes_match[1].to_i
    sleep = attributes_match[2].to_i

    reindeers << Reindeer.new(name, velocity, endurance, sleep) 
end

reindeers.each {|r| r.print}

maxDistance = 0
2503.times do |second|

    puts "------Second: #{second+1}------"
    reindeers.each do |reindeer|
        if reindeer.stamina > 0
            reindeer.distance += reindeer.velocity 
            reindeer.stamina -= 1
            reindeer.alarm = reindeer.sleep if reindeer.stamina == 0
        elsif reindeer.alarm > 0
            reindeer.alarm -= 1
        end

        if reindeer.stamina <= 0 and reindeer.alarm <= 0
            reindeer.stamina = reindeer.endurance
        end

        if reindeer.distance > maxDistance
            maxDistance = reindeer.distance
        end
    end

    reindeers.each do |reindeer|
        reindeer.points += 1 if reindeer.distance == maxDistance
        puts "#{reindeer.name} scored #{reindeer.points}"
    end
    puts
    
end


