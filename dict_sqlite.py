import sqlite3

conn = sqlite3.connect('dictionary.db')

cur = conn.cursor()

input_word = input("Enter Word: ")

while(input_word != 'exit'):

    meaning = conn.execute(
        f'SELECT * FROM dict WHERE word="{input_word}"'
    )

    if cur is None:
        print("Meaning not found!")
    else:
        for i in meaning:
            print(i)
    input_word = input("Enter Word: ")
