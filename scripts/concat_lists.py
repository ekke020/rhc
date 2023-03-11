import sys

def load_values(filepath):
  content = []
  with open(filepath, 'r') as file:
    for line in file:
        content.append(line.strip())
  return set(content)

try:
  file_1 = sys.argv[1]
  file_2 = sys.argv[2]
except:
   print(f'Requires 2 arguments but only {len(sys.argv) - 1} was provided')
   exit(1)

file_1_content = []

try:
  set_1 = load_values(file_1)
  set_2 = load_values(file_2)
except FileNotFoundError as e:
   print(f'No file at given path: {e.filename}')
   exit(1)

combined = set_1.union(set_2)
write_file = 'output.txt'
try:
  with open(write_file, 'w') as file:
    file.write(combined.pop())
    for password in combined:
      file.write(f'\n{password}')
except:
  print(f'Unable to write to file: {write_file}')
  exit(1)

print(f'Successfully combined {file_1} with {file_2} and produced {write_file}')