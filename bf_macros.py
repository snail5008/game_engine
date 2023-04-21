import sys

with open(sys.argv[1], 'r') as f:
    contents = f.read()

final_string = ""

i = 0
while True:
    if i >= len(contents):
        break

    if contents[i] != '~':
        final_string += contents[i]
        i += 1
    else:
        i += 1
        if contents[i] == '&' or contents[i] == '^':
            reptype = contents[i]

            i += 2
            char = contents[i - 1]
            string = ""
            while contents[i] != '\n':
                string += contents[i]
                i += 1
                
            if reptype == '&':
                rep_times = int(string)
            elif reptype == '^':
                rep_times = int(string, base=16)

            final_string += (char * rep_times)
            continue
        while contents[i] != '\n':
            final_string += f"{'+' * ord(contents[i])}>\n"
            i += 1
        i += 1

print(final_string)
