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
    matrix = Array.new($locations.size) {Array.new($locations.size) {0}} 
    $input.each do |line|
        m_loc = line.scan(/[a-zA-Z]+/)
        outer = $locations.find_index(m_loc[0])
        inner = $locations.find_index(m_loc[2])
        matrix[outer][inner] = line.scan(/[0-9]+/)[0].to_i 
        matrix[inner][outer] = line.scan(/[0-9]+/)[0].to_i 
    end
    matrix
end

$locations = get_locations()
$matrix = build_matrix()

$matrix.each {|array| p array}
