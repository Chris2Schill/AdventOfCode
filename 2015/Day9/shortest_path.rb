# Dijkstra's Algorithm implemented in ruby
$locations = Array.new
$input = Array.new

File.readlines('input.txt').each do |line|
    $input << line
end

def get_locations
    locations = Array.new
    $input.each do |line|
        m_loc = line.scan(/[a-zA-Z]+/) 
        m_loc.delete('to')
        m_loc.each {|location| locations << location unless locations.include?(location)}
    end
    locations
end


def build_matrix
    matrix = Array.new($locations.size) {Array.new($locations.size) {9999}} 
    $input.each do |line|
        m_loc = line.scan(/[a-zA-Z]+/)
        outer = $locations.find_index(m_loc[0])
        inner = $locations.find_index(m_loc[2])
        matrix[outer][inner] = line.scan(/[0-9]+/)[0].to_i 
    end
    matrix
end

$locations = get_locations()
$matrix = build_matrix()

distance = Array.new($locations.size) {0}
visited = Array.new($locations.size) {0}
preD = Array.new($locations.size) {0}

distance = $matrix[0]
distance[0] = 0
visited[0] = 1
next_node = 0

min = 999

$matrix.each {|c| p c}

$locations.size.times do |i|
    $locations.size.times do |j|
        if (min > distance[j] and visited[j] != 1)                
            min = distance[j] 
            next_node = j
        end
    end
    visited[next_node] = 1
    $locations.size.times do |c|
        if visited[c] != 1
            if min+$matrix[next_node][c] < distance[c]
                distance[c] = min + $matrix[next_node][c]
                preD[c] = next_node
            end
        end
    end
end

$locations.size.times do |i|
    print "| #{distance[i]}"
end
puts '|'

puts 134+4+105+68+11+115+127

$locations.size.times do |i| 
    print "Path = #{i}"
    j = i
    begin
        j = preD[j]
        print " <- #{j}"
    end while j != 0
    puts
end
