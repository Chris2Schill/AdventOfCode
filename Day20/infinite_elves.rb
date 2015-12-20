$input = 36000000
$houses = Array.new(2000000){0}
$house_over_input = Array.new

class Elf
    attr_accessor :id
    @@total_elves = 0
    def initialize
        @@total_elves += 1
        @id = @@total_elves
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

end

2000000.times do
    Elf.new.deliver
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
p $house_over_input.first


