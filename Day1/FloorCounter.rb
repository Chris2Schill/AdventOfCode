floor = 0
File.open("input.txt") do |f|
    f.each_char do |c|
        if c == '(' then floor += 1;
        else if c == ')' then floor -= 1; 

        end
        end
    end
end

print floor
