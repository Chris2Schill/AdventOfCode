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
            if floor == -1
                return charPosition
            end
        end
    end
end

print findPosition
