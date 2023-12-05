class Mapping:
    def __init__(self, dest_start, source_start, range_length):
        self.dest_start = dest_start
        self.source_start = source_start
        self.range_length = range_length

    def map(self, source):
        if self.source_start <= source < self.source_start + self.range_length:
            return self.dest_start + (source - self.source_start)
        return source

def find_lowest_location(seed_numbers, maps):
    locations = set(seed_numbers)
    
    for _ in range(10):  # Repeat the process multiple times (adjust as needed)
        for seed_number in locations.copy():
            for map in maps:
                for mapping in map:
                    locations.add(mapping.map(seed_number))

    return min(locations, default=0)

def main():
    # Initial seed numbers
    seed_numbers = [79, 14, 55, 13]

    # Seed-to-soil map
    seed_to_soil_map = [
        Mapping(dest_start=50, source_start=98, range_length=2),
        Mapping(dest_start=52, source_start=50, range_length=48),
    ]

    # Soil-to-fertilizer map
    soil_to_fertilizer_map = [
        Mapping(dest_start=0, source_start=15, range_length=37),
        Mapping(dest_start=37, source_start=52, range_length=2),
        Mapping(dest_start=39, source_start=0, range_length=15),
    ]

    # Fertilizer-to-water map, and so on...

    # Combine all maps
    all_maps = [seed_to_soil_map, soil_to_fertilizer_map]  # Add other maps here

    # Find the lowest location number
    lowest_location = find_lowest_location(seed_numbers, all_maps)

    print(f"The lowest location number is: {lowest_location}")

if __name__ == "__main__":
    main()
