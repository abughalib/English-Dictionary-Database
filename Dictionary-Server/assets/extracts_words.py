import json

word_file = open('dict_word.txt', mode='w', encoding='utf-8')

for i in range(65, 91):
  file = open(f"D{chr(i)}_mod.json", mode='r', encoding='utf-8')

  file_str = ""
  for i in file.readlines():
    file_str += i
  
  file_json = json.loads(file_str)

  for i in file_json.keys():
    word_file.write(f"{i} ")

  file.close()

word_file.close()