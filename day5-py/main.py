import re
import threading


results = []
def solve(seed_range, plant_maps):
    min_seed = seed_range[-1] + 1
    for seed in seed_range:
        pointer = int(seed)
        for map_name in plant_maps:
            for cover in plant_maps[map_name]:
                dest_start = int(cover[0])
                src_start = int(cover[1])
                length = int(cover[2])

                if pointer >= src_start and pointer < src_start + length:
                    pointer = dest_start + (pointer - src_start)
                    break
        if pointer < min_seed:
            min_seed = pointer
    results.append(min_seed)
    print('THREAD DONE!')

with open('input.txt', 'r') as f:
    lines = f.readlines()

seeds  = lines[0].split(':')[1].strip().replace('\n','').split(' ')
seeds = [int(x) for x in seeds]
seeds_expanded = []

for i in range(0, len(seeds), 2):
    seeds_expanded.append(range(seeds[i], seeds[i] + seeds[i + 1]))

print(seeds_expanded)

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


seed_locations = []



threads = []
for seed_range in seeds_expanded:
    
    thread = threading.Thread(target=solve, args=(seed_range, plant_maps))
    threads.append(thread)
    thread.start()

for thread in threads:
    thread.join()
# for seed_range in seeds_expanded:
#     print(seed_range)
#     for seed in seed_range:
#         pointer = int(seed)
#         for map_name in plant_maps:
#             for cover in plant_maps[map_name]:
#                 dest_start = int(cover[0])
#                 src_start = int(cover[1])
#                 length = int(cover[2])
                
#                 if pointer >= src_start and pointer < src_start + length:
#                     pointer = dest_start + (pointer - src_start)
#                     break
#         seed_locations.append(pointer)

print(min(results))





