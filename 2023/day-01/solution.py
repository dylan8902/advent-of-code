#!/usr/bin/python3


import os
import re

# Part 1
total = 0
with open(os.path.join(os.path.dirname(__file__), 'input.txt'), 'r') as f:
    for line in f:
        first = re.search(r"[0-9]", line).group(0)
        last = re.search(r"[0-9]", line[::-1]).group(0)
        total += int(str(first) + str(last))

print("Part 1 answer:")
print(total)

# Part 2
total = 0
number_replacements = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9"
}

with open(os.path.join(os.path.dirname(__file__), 'input.txt'), 'r') as f:
    for line in f:
        print(line)
        for word, number in number_replacements.items():
            line = line.replace(word, number)
        print(line)
        first = re.search(r"[0-9]", line).group(0)
        last = re.search(r"[0-9]", line[::-1]).group(0)
        print(str(first) + str(last))
        total += int(str(first) + str(last))

print("Part 2 answer:")
print(total)