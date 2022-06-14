import psycopg2

words = open("../assets/dict_words.txt", encoding="utf-8", mode='r')

for i in words.readlines():
  print(i, end='')