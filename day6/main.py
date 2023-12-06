import re
import math
with open('input1.txt', 'r') as f:
    lines = f.readlines()

def get_ways_to_win(time, dist_to_beat):
    possible_winners = []
    for hold_time in range(1, time):
        possible_dist = (time - hold_time) * hold_time
        if possible_dist > dist_to_beat:
            possible_winners.append(hold_time)
    return possible_winners

times = re.findall(r'\d+', lines[0])
time_extended = int(''.join(times))

times = [int(x) for x in times]
distances = re.findall(r'\d+',lines[1])
dist_extended = int(''.join(distances))
distances = [int(x) for x in distances]

race_stats = zip(times, distances)
possible_ways_acc = []
for (i, time) in enumerate(times):
    possible_winners = []
    for hold_time in range(1, time):
        possible_dist = (time - hold_time) * hold_time
        if possible_dist > distances[i]:
            possible_winners.append(hold_time)
    possible_ways_acc.append(len(possible_winners))
    
print(math.prod(possible_ways_acc))

print(time_extended)
print(dist_extended)

p2_winners = get_ways_to_win(time_extended, dist_extended)
print(len(p2_winners))