import re
import threading



with open('input.txt', 'r') as f:
    lines = f.readlines()

seeds  = lines[0].split(':')[1].strip().replace('\n','').split(' ')
seeds = [int(x) for x in seeds]
seeds_expanded = []

for i in range(0, len(seeds), 2):
    seeds_expanded.append((seeds[i], seeds[i] + seeds[i + 1]))



current_map = ''
plant_maps = {}
for line in lines[1:]:
    if line[0].isalpha():
        current_map = line.replace(':', '').replace('\n', '')
        
    if line[0].isnumeric():
        numbers = re.findall(r'\d+', line)
        if current_map in plant_maps:
            plant_maps[current_map].append(numbers)
        else:
            plant_maps[current_map] = []
            plant_maps[current_map].append(numbers)



for map in plant_maps:
    mapped_ranges = []

    while seeds_expanded:
        seed_start, seed_stop = seeds_expanded.pop()
        done_mapping = False
        
        for dest, src, sz in plant_maps[map]:
            src = int(src)
            dest = int(dest)
            sz = int(sz)

            overlap_start = max(seed_start, int(src))
            overlap_stop = min(seed_stop, int(src) +sz)

            if overlap_start < overlap_stop:
                
                mapped_ranges.append((dest + (overlap_start - src), (dest + (overlap_stop - src))))
        
                if seed_start < overlap_start:
                    seeds_expanded.append((seed_start, overlap_start))
                if overlap_stop < seed_stop:
                    seeds_expanded.append((seed_stop, overlap_stop))
                done_mapping = True
                break
        if not done_mapping:
            mapped_ranges.append((seed_start, seed_stop))
    seeds_expanded = mapped_ranges


print(min(seeds_expanded)[0])





