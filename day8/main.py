import math
from itertools import cycle
import re

with open('input3.txt', 'r') as f:
    lines = f.readlines()
# print(lines)

pattern = re.compile(r'\b([A-Za-z|]+)\b')



instructions = lines[0]
path = {}
starting_key = ''
for (i,line) in enumerate(lines[1:]):
    matches = pattern.findall(line)
    print(matches)
    if matches:
        path[matches[0]] = (matches[1], matches[2])
    if i == 1:
        starting_key = matches[0]


instructions = [x for x in instructions if x == 'L' or x == 'R']
key = 'AAA'

idx = 0
count = 0
while key != 'ZZZ':

    if instructions[idx] == 'L':
        key = path[key][0]
    else:
       key = path[key][1]

    
    count+=1
    idx = (idx + 1) % len(instructions)
        
print(count)

possble_keys = [key for key in path.keys() if key.endswith('A')]

key1 = possble_keys[0]
key2 = possble_keys[1]

count = 0
idx = 0
# while not all(key.endswith('Z') for key in possble_keys):
    
#     if instructions[idx] == 'L':
#         key_loc = 0
#     else:
#         key_loc = 1
        
#     for (i, key) in enumerate(possble_keys):
        
#         possble_keys[i] = path[key][key_loc]

#     count+=1
#     idx = (idx + 1) % len(instructions)
current_match_list = [key for key in path.keys() if key.endswith('A')]
total_iteration = 0
iteration = 0
while not all(isinstance(element, int) for element in current_match_list):
    for idx in range(len(current_match_list)):
        element = current_match_list[idx]
        if isinstance(element, int): continue
        if element.endswith('Z'):
            current_match_list[idx] = int(total_iteration)
            continue
        if instructions[iteration] == "R":
            element = path[element][1]
        if instructions[iteration] == "L":
            element = path[element][0]
        current_match_list[idx] = element
    total_iteration += 1
    iteration = (iteration + 1) % len(instructions)
result = math.lcm(*current_match_list)
print(f"Part 2: {result}")




