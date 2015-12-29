$input = 36000000
$houses = Array.new
$house_over_input = Array.new

class Elf

    attr_accessor :id, :presents
    @@elves = Array.new

    def initialize
        @@elves << self
        @id = @@elves.size
        @presents = @id*10
    end

    def deliver 
        i = @id
        while i < $houses.size
            $houses[i] = $houses[i] + @presents
            if $houses[i] >= $input 
                $house_over_input << i
            end
            i += @id
        end
    end

    def deliver2
        presents = 0
        if @@elves.size % 2 == 0 and @@elves.size % 3 == 0
            @@elves.each do |elf|
                presents += elf.presents if @@elves.size % elf.id == 0
            end
            $houses[@@elves.size] = presents
        else
            $houses[@@elves.size] = 0
        end
    end
end

250000.times do
    Elf.new.deliver2
end

def test
    test_passed = true
    test_passed = false if $houses[1] != 10
    test_passed = false if $houses[2] != 30
    test_passed = false if $houses[3] != 40
    test_passed = false if $houses[4] != 70
    test_passed = false if $houses[5] != 60
    test_passed = false if $houses[6] != 120
    test_passed = false if $houses[7] != 80
    test_passed = false if $houses[8] != 150
    test_passed = false if $houses[9] != 130
    if test_passed
        puts "Tests Passed"
    else
        puts "Tests Failed"
    end
end

test

$houses.size.times do |i|
    puts "House #{i+1}: #{$houses[i+1]}" if $houses[i+1] != 0
end

