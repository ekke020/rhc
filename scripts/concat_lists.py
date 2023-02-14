import sys

def load_values(filepath):
  content = []
  with open(filepath, 'r') as file:
    for line in file:
        content.append(line.strip())
  return set(content)

def write_values(combined):
  with open('test.txt', 'w') as file:
    for password in combined:
      file.write(password + '\n')

try:
  file_1 = sys.argv[1]
  file_2 = sys.argv[2]
except:
   print("Missing arguments ")

file_1_content = []

set_1 = load_values(file_1)
set_2 = load_values(file_2)

combined = set_1.union(set_2)
write_values(combined)

