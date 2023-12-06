import re
import math
with open('input1.txt', 'r') as f:
    lines = f.readlines()



times = re.findall(r'\d+', lines[0])
times = [int(x) for x in times]
distances = re.findall(r'\d+',lines[1])
distances = [int(x) for x in distances]

race_stats = zip(times, distances)
possible_ways_acc = []
for (i, time) in enumerate(times):
    # print(time, distances[i])
    possible_winners = []
    for hold_time in range(1, time):
        possible_dist = (time - hold_time) * hold_time
        if possible_dist > distances[i]:
            possible_winners.append(hold_time)
    possible_ways_acc.append(len(possible_winners))
    
print(math.prod(possible_ways_acc))