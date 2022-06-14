import json
import re


def trim_json_str(json_str, pattern, replacement):
  json_str = re.sub(pattern, replacement, json_str)
  return json_str


for i in range(66, 91):

  file = open(f"D{chr(i)}.json", mode="r", encoding="utf-8")
  file_write = open(f"D{chr(i)}_mod.json", mode="w", encoding="utf-8")

  file_str = ""

  for i in file.readlines():
    file_str += i

  file_json = json.loads(file_str)

  json_meaning_str = "{"+'\n'

  for i in file_json.keys():
    json_meaning_str += f'   "{i}":' + '{' + '\n'
    meaning_lst = []
    keyword_lst = []
    if not file_json[i]["MEANINGS"]:
      json_meaning_str += r'      "meaning": {'+'\n'
      json_meaning_str += r'      "def":'+str(meaning_lst)+','+'\n'
      json_meaning_str += r'      "keywords":'+str(keyword_lst)+'\n'
      json_meaning_str += '    },' + '\n'
      antonyms = file_json[i]["ANTONYMS"]
      json_meaning_str += f'    "antonyms":{str(antonyms)},'+'\n'
      synonyms = file_json[i]["SYNONYMS"]
      json_meaning_str += f'    "synonyms":{str(synonyms)}'
      json_meaning_str += '  },'+'\n'
      continue
    json_meaning_str += r'      "meaning": {'+'\n'
    for j in file_json[i]["MEANINGS"]:
      count = 0
      for k in file_json[i]["MEANINGS"][j]:
        if count <= 1:
          meaning_lst.append(k)
        else:
          keyword_lst.extend(k)
        count += 1
    json_meaning_str += r'      "def":'+str(meaning_lst)+','+'\n'
    json_meaning_str += r'      "keywords":'+str(keyword_lst)+'\n'

    json_meaning_str += '      },' + '\n'
    antonyms = file_json[i]["ANTONYMS"]
    json_meaning_str += f'    "antonyms":{str(antonyms)},'+'\n'
    synonyms = file_json[i]["SYNONYMS"]
    json_meaning_str += f'    "synonyms":{str(synonyms)}'
    json_meaning_str += '\n'+'  },'+'\n'

  json_meaning_str += " }"

  # trim_json_str(json_meaning_str, '"-', "")
  # trim_json_str(json_meaning_str, "['", '["')
  # trim_json_str(json_meaning_str, "']", '"]')
  # trim_json_str(json_meaning_str, '("', '(')
  # trim_json_str(json_meaning_str, ': "', ": ")
  # trim_json_str(json_meaning_str, '""', '"')
  # trim_json_str(json_meaning_str, ", '", ', "')
  # trim_json_str(json_meaning_str, "',", '",')
  # trim_json_str(json_meaning_str, ''''"''''', '"')

  file_write.write(json_meaning_str)

  file.close()
  file_write.close()
