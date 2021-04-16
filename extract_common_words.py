import sqlite3


conn = sqlite3.connect('words.db')
cur = conn.cursor()


def remove_not_word():
    for word in cur.execute('SELECT * FROM words'):
        if (word[0].isalpha() == False):
            cur.execute('DELETE FROM words WHERE word=?', (word[0], ))
            print(f"{word[0]} is not a word! && Removing it.")
        else:
            pass


for i in range(0, 83253):
    remove_not_word()

conn.commit()
