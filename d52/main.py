output = 0

file = 'input2.txt'
file = 'input.txt'

lines = [line for line in open(file).read().rstrip().split('\n')]

sts = False
stf = False
ftw = False
wtl = False
ltt = False
tth = False
htl = False

seed_to_soil = []
soil_to_fertilizer = []
fertilizer_to_water = []
water_to_light = []
light_to_temperature = []
temperature_to_humidity = []
humidity_to_location = []

for line in lines:

    if line.split(':')[0] == 'seeds':
        seeds = line.split(':')[1].rstrip().strip().split(' ')

    if line == 'seed-to-soil map:' or sts == True:
        if line.strip() == '':
            sts = False
            continue

        if sts:
            seed_to_soil.append(line.split(' '))
            if line.strip() == '':
                sts = False
                continue
        else:
            sts = True

    if line == 'soil-to-fertilizer map:' or stf == True:
        if line.strip() == '':
            stf = False
            continue

        if stf:
            soil_to_fertilizer.append(line.split(' '))
            if line.strip() == '':
                stf = False
                continue
        else:
            stf = True

    if line == 'fertilizer-to-water map:' or ftw == True:
        if line.strip() == '':
            ftw = False
            continue

        if ftw:
            fertilizer_to_water.append(line.split(' '))
            if line.strip() == '':
                ftw = False
                continue
        else:
            ftw = True
    
    if line == 'water-to-light map:' or wtl == True:
        if line.strip() == '':
            wtl = False
            continue

        if wtl:
            water_to_light.append(line.split(' '))
            if line.strip() == '':
                wtl = False
                continue
        else:
            wtl = True
    
    if line == 'light-to-temperature map:' or ltt == True:
        if line.strip() == '':
            ltt = False
            continue

        if ltt:
            light_to_temperature.append(line.split(' '))
            if line.strip() == '':
                ltt = False
                continue
        else:
            ltt = True
    
    if line == 'temperature-to-humidity map:' or tth == True:
        if line.strip() == '':
            tth = False
            continue

        if tth:
            temperature_to_humidity.append(line.split(' '))
            if line.strip() == '':
                tth = False
                continue
        else:
            tth = True
    
    if line == 'humidity-to-location map:' or htl == True:
        if line.strip() == '':
            htl = False
            continue

        if htl:
            humidity_to_location.append(line.split(' '))
            if line.strip() == '':
                htl = False
                continue
        else:
            htl = True
    
def get_to_location(seed, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location):
    
    sts = False
    for map in seed_to_soil:
        if int(seed) >= int(map[1]) and int(seed) <= int(map[1]) + int(map[2]):
            soil = int(map[0]) + int(seed) - int(map[1])
            sts = True
            break
    if not sts:
        soil = int(seed)
    
    stf = False
    for map in soil_to_fertilizer:
        if int(soil) >= int(map[1]) and int(soil) <= int(map[1]) + int(map[2]):
            fertilizer = int(map[0]) + int(soil) - int(map[1])
            stf = True
            break
    if not stf:
        fertilizer = soil

    ftw = False
    for map in fertilizer_to_water:
        if int(fertilizer) >= int(map[1]) and int(fertilizer) <= int(map[1]) + int(map[2]):
            water = int(map[0]) + int(fertilizer) - int(map[1])
            ftw = True
            break
    if not ftw:
        water = fertilizer
    
    wtl = False
    for map in water_to_light:
        if int(water) >= int(map[1]) and int(water) <= int(map[1]) + int(map[2]):
            light = int(map[0]) + int(water) - int(map[1])
            wtl = True
            break
    if not wtl:
        light = seed
    
    ltt = False
    for map in light_to_temperature:
        if int(light) >= int(map[1]) and int(light) <= int(map[1]) + int(map[2]):
            temperature = int(map[0]) + int(light) - int(map[1])
            ltt = True
            break
    if not ltt:
        temperature = light
    
    tth = False
    for map in temperature_to_humidity:
        if int(temperature) >= int(map[1]) and int(temperature) <= int(map[1]) + int(map[2]):
            humidity = int(map[0]) + int(temperature) - int(map[1])
            tth = True
            break
    if not tth:
        humidity = temperature
    
    htl = False
    for map in humidity_to_location:
        if int(humidity) >= int(map[1]) and int(humidity) <= int(map[1]) + int(map[2]):
            location = int(map[0]) + int(humidity) - int(map[1])
            htl = True
            break
    if not htl:
        location = humidity
    
    return location

def location_to_soil(location, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location):

    for map in humidity_to_location:
        #print('check:', location, 'htl:', map)
        if location >= int(map[0]) and location <= int(map[0]) + int(map[2]):
            humidity = int(map[1]) + (location - int(map[0]))
            break
        humidity = location
    
    #print('humidity:', humidity)
    
    for map in temperature_to_humidity:
        #print('check:', humidity, 'tth:', map)
        if humidity >= int(map[0]) and humidity <= int(map[0]) + int(map[2]):
            temperature = int(map[1]) + (humidity - int(map[0]))
            break
        temperature = humidity
    
    #print('temperature:', temperature)
    
    for map in light_to_temperature:
        #print('check:', humidity, 'ltt:', map)
        if temperature >= int(map[0]) and temperature <= int(map[0]) + int(map[2]):
            light = int(map[1]) + (temperature - int(map[0]))
            break
        light = temperature
    
    #print('light:', light)
    
    for map in water_to_light:
        if light >= int(map[0]) and light <= int(map[0]) + int(map[2]):
            water = int(map[1]) + (light - int(map[0]))
            break
        water = light
    
    for map in fertilizer_to_water:
        if water >= int(map[0]) and water <= int(map[0]) + int(map[2]):
            fertilizer = int(map[1]) + (water - int(map[0]))
            break
        fertilizer = water
    
    for map in soil_to_fertilizer:
        if fertilizer >= int(map[0]) and fertilizer <= int(map[0]) + int(map[2]):
            soil = int(map[1]) + (fertilizer - int(map[0]))
            break
        soil = fertilizer
    
    for map in seed_to_soil:
        if soil >= int(map[0]) and soil <= int(map[0]) + int(map[2]):
            seed = int(map[1]) + (soil - int(map[0]))
            break
        seed = soil
    
    return seed
    
def valid_seed(number, seeds):
    if number == 0:
        return False
    cntr = 0
    while cntr < len(seeds):
        #print('number', number, 'from:', str(int(seeds[cntr])), 'to:', str(int(seeds[cntr]) + int(seeds[cntr + 1])))
        if number >= int(seeds[cntr]) and number <= int(seeds[cntr]) + int(seeds[cntr + 1]):
            print('VALID!!')
            return True
        cntr += 2
    return False

# part 1
locations = []
for seed in seeds:
    locations.append(get_to_location(seed, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location))
print(min(locations))

# part 2
check = 0
cntr = 0
while not valid_seed(check, seeds):
    cntr += 1
    check = location_to_soil(cntr, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location)
    if cntr % 100_000 == 0:
        print(cntr)
print(cntr)
