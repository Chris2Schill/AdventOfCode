def findPosition
    floor = 0
    charPosition = 0
    File.open("input.txt") do |f|
        f.each_char do |c|
            if c == '('
                floor += 1
            elsif c == ')'
                floor -= 1
            end
            charPosition += 1
            return charPosition if floor == -1
        end
    end
end

puts findPosition
