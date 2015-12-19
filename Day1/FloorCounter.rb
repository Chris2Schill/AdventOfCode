floor = 0
File.open("Day1/input.txt") do |f|
    f.each_char do |c|
        if c == '('
            floor += 1
        elsif c == ')'
            floor -= 1
        end
    end
end

puts floor
