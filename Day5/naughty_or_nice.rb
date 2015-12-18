count = 0
File.readlines('input.txt').each do |line|
       # Contains 3 or more vowels
    if line.match(/[aeiou].*?[aeiou].*?[aeiou]/) and 
           # Two or more consecutive characters
            line.match(/\S*(.)\1\S*/) and line.match(/^((?!ab|cd|pq|xy).)*$/)
            # Does not contain 'ab', 'cd', 'pq', or 'xy'
        count += 1
    end
end
puts count
