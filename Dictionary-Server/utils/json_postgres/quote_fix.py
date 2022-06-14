#!/usr/bin/python
# replace.py
import sys

# Replace string in a file (in place)

pattern_to_be_replaced = [(r"['", r'["'), (r"']", r'"]'), (r"', ", r'", '),
                          (r'""', r'"'), (r': "', r": "), (r"', '",
                                                           r'", "'), (r'")', r')'),
                          (r'"-', r" -"), (r'" -', r"-"), (r"\'", r"'")]

for i in range(67, 91):
  for pattern in pattern_to_be_replaced:
    match = pattern[0]
    replace = pattern[1]
    filename = f"D{chr(i)}_mod.json"

    print(f"Replacing strings in {filename}")
    with open(filename, "r") as f:
      data = f.read().replace(match, replace)

    with open(filename, "w") as f:
      f.write(data)
