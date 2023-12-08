import re

with open('input3.txt', 'r') as f:
    lines = f.readlines()


pattern = re.compile(r'\b([A-Za-z]+)\b')



instructions = lines[0]
path = {}
starting_key = ''
for (i,line) in enumerate(lines[1:]):
    matches = pattern.findall(line)
    if matches:
        path[matches[0]] = (matches[1], matches[2])
    if i == 1:
        starting_key = matches[0]
print(path)
print(instructions)

# print(path)

print(instructions)
instructions = [x for x in instructions if x == 'L' or x == 'R']
key = starting_key

idx = 0
count = 0
while key != 'ZZZ':

    assert instructions[idx] ==  'L' or instructions[idx] == 'R', instructions
    print(key, instructions[idx])
    if instructions[idx] == 'L':
        key = path[key][0]
    else:
       key = path[key][1]

    
    count+=1
    idx += 1
    if idx == len(instructions):
        idx = 0
print(count)


