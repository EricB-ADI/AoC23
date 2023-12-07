# with open('input1.txt', 'r') as f:
#     lines = f.readlines()

# lines = [x.strip() for x in lines]

# card_order = {'A': 12,
#          'K': 11, 
#          'Q': 10, 
#          'J': 9, 
#          'T': 8, 
#          '9': 7, 
#          '8': 6, 
#          '7': 5, 
#          '6': 4, 
#          '5': 3, 
#          '4': 2, 
#          '3': 1, 
#          '2': 0}


# def get_unique_count(hand):
#     counts = {}
#     for c in hand.strip():

#         if c in counts:
#             counts[c] += 1
#         else:
#             counts[c] = 1

#     return counts


# def get_rank(count):
    
#     if len(count) == 1:
#         # five of a kind
#         return 7
#     elif len(count) == 2:
#         # four of a kind
#         return 6
#     elif len(count) == 5:
#         # high card
#         return 1

#     count_values = count.values()
#     if 3 in count_values and len(count) == 2:
#         # full house
#         return 5
#     elif 3 in count_values and len(count) == 3:
#         # three of a kind
#         return 4
#     elif 2 in count_values and len(count) == 4:
#         # one pair
#         return 2
#     else:
#         # two pair
#         return 3


# hand_counts = []
# ranks = []
# for line in lines:
#     hand, bet = line.split()
#     count = get_unique_count(hand)
#     hand_counts.append(count)
#     rank = get_rank(count)
#     ranks.append((rank, hand, bet))


# sub_ranks = {}

# for rank in ranks:

#     if rank[0] not in sub_ranks:
#         sub_ranks[rank[0]] = []
#     sub_ranks[rank[0]].append(rank)

# expanded = [] * len(sub_ranks)

# for sub_ranking in sub_ranks:
#     sub_ranks[sub_ranking] = sorted(sub_ranks[sub_ranking], key=lambda x: [card_order[char] for char in x[1]])
#     expanded.insert(sub_ranking, sub_ranks[sub_ranking])    



# expanded_full = sorted(expanded, key=lambda x: x[0])

# flattened_list = [element for sublist in expanded_full for element in sublist]
# # print(flattened_list)
# total_winnings = 0
# for i, hand in enumerate(flattened_list):
#     # print(hand)
#     total_winnings += (i + 1) * int(hand[2])
# print(total_winnings)

hands = open("input1.txt").read().splitlines()
hands = [hand.split(" ") for hand in hands]

cards = list("AKQJT98765432")
cardDict = {}
for i, card in enumerate(cards[::-1]):
    cardDict[card] = i+1

def getType(hand):
    # 7: five of a kind, 6: four of a kind, 5: full house, etc.
    uniques = list(set(hand))
    noofUniques = len(uniques)

    if noofUniques == 1:
        return 7
    if noofUniques == 2:
        countUnique = hand.count(uniques[0])
        if countUnique == 1 or countUnique == 4:
            return 6
        return 5
    if noofUniques == 3:
        structure = sorted([hand.count(uniques[i]) for i in range(3)])
        if structure == [1,1,3]:
            return 4
        return 3
    if noofUniques == 4:
        return 2
    return 1

def getHandValue(hand):
    # assigns a value to a hand based on each card (right to left in increasing importance) and its type
    total = 0
    for i, card in enumerate(hand[::-1]):
        total += cardDict[card] * (16**i)
    total += getType(hand) * (16**5)
    return total

sortedHands = sorted(hands, key=lambda hand: getHandValue(hand[0]))

total = 0
for i in range(len(hands)):
    total += (i + 1) * int(sortedHands[i][1])

print(total)




hands = open("input1.txt").read().splitlines()
hands = [hand.split(" ") for hand in hands]

cards = list("AKQT98765432J")
cardDict = {}
for i, card in enumerate(cards[::-1]):
    cardDict[card] = i+1

def getType(hand):
    # 7: five of a kind, 6: four of a kind, 5: full house, etc.
    uniques = list(set(hand))
    noOfUniques = len(uniques)

    if "J" in hand and noOfUniques != 1:
        bestReplacement = max(hand.replace("J",""), key=lambda card: hand.count(card))
        hand = hand.replace("J",bestReplacement)
        uniques = list(set(hand))
        noOfUniques -= 1

    if noOfUniques == 1:
        return 7
    if noOfUniques == 2:
        countUnique = hand.count(uniques[0])
        if countUnique == 1 or countUnique == 4:
            return 6
        return 5
    if noOfUniques == 3:
        structure = sorted([hand.count(uniques[i]) for i in range(3)])
        if structure == [1,1,3]:
            return 4
        return 3
    if noOfUniques == 4:
        return 2
    return 1

def getHandValue(hand):
    # assigns a value to a hand based on each card (right to left in increasing importance) and its type
    total = 0
    for i, card in enumerate(hand[::-1]):
        total += cardDict[card] * (16**i)
    total += getType(hand) * (16**5)
    return total

sortedHands = sorted(hands, key=lambda hand: getHandValue(hand[0]))

total = 0
for i in range(len(hands)):
    total += (i + 1) * int(sortedHands[i][1])

print(total)