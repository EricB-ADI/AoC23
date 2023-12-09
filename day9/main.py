import numpy
with open('input.txt', 'r') as f:
    lines = f.readlines()




def parse(values):
    
    matrix = [values]

    while not all(x == 0 for x in values):
        values = [second - first for first, second in zip(values, values[1:])]
        matrix.append(values)

    return matrix

def retrieve_history(values):
    matrix = parse(values)

    offset = 0
    for idx in range(len(matrix)-1, -1, -1):
        matrix[idx].append(matrix[idx][-1] + offset)
        offset = matrix[idx][-1]

    return offset

def retrieve_history_backwards(line):
    matrix = parse(line)

    offset = 0
    for idx in range(len(matrix)-1, -1, -1):
        matrix[idx].insert(0, matrix[idx][0] - offset)
        offset = matrix[idx][0]

    return offset

history_values = []
for line in lines:
    line_parsed = line.split(' ')
    values = [int(x) for x in line_parsed]
    history = retrieve_history(values)
    history_values.append(history)


history_values_pt2 = []
for line in lines:
    line_parsed = line.split(' ')
    values = [int(x) for x in line_parsed]
    history = retrieve_history_backwards(values)
    history_values_pt2.append(history)
    
print(sum(history_values))
print(sum(history_values_pt2))