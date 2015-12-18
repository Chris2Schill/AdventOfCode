#separate number into groups of the same number -> into an array
#iterate the list and append to empty string the length of each group, then the number in the group
#repeat for each element in array.
#return string

def separate_number_into_groups num_as_string
    num_groups = Array.new
    str = ''
    num_as_string.each_char do |c|
        if str[0] != c and str != ''
            num_groups << str
            str = ''
        end
        str += c
    end
    num_groups << str
end

def test_number_separator
    group1 = separate_number_into_groups '11333411'
    group2 = separate_number_into_groups '1'
    if group1[0] != '11' and group1[1] != '333' and group1[2] != '4' and group1[3] != '11'
        puts 'Test_Number_Separator Failed'
        return
    elsif group2[0] != '1'
        puts 'Test_Number_Separator Failed'
        return
    end
    puts 'Test_Number_Separator Passed'
end

def test_create_new_string
    str1 = create_new_string_from '1'
    str2 = create_new_string_from '11' 
    str3 = create_new_string_from '21'
    str4 = create_new_string_from '1211'
    str5 = create_new_string_from '111221'
    puts "Test_Create_New_String Failed" if str1 != '11' || 
            str2 != '21' ||
            str3 != '1211' ||
            str4 != '111221' ||
            str5 != '312211'
    puts "Test_Create_New_String Passed"
end

def create_new_string_from input
    array = separate_number_into_groups(input) 
    new_string = ''
    array.each do |group|
        new_string += group.length.to_s
        new_string += group[0].to_s
    end
    new_string
end


test_number_separator
test_create_new_string

input = '1113222113'
50.times do 
    puts input.length
    input = create_new_string_from input
end
puts input.length
