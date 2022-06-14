import re


def remove_last_comma():
    file = open('example.json', mode='r')
    num_of_line = sum(1 for line in file)
    file.close()
    return num_of_line


file = open('example.json', mode='r')
wrt = open('reordered_json.json', mode='w')

wrt.write("{")

idex = 0

for line in file.readlines():
    reordered_line = f'''"{idex}":'''
    dual_quote_close = True
    for char in line:
        if char == ',':
            if dual_quote_close:
                reordered_line += ',\n'
        elif char == "'":
            if dual_quote_close:
                dual_quote_close = False
            else:
                dual_quote_close = True
            reordered_line += '"'
        else:
            reordered_line += char

    # I would recoment you to remove the last comma by yourself from reorderd_json
    # This function would cost twice time

    num_of_line = remove_last_comma()
    if idex < num_of_line-1:
        reordered_line += ','

    # Use the below instead to save time.
    # and manually remove the last comma
    # reordered_line += ','

    reordered_line = re.sub("( None)", '"None"', reordered_line)

    wrt.write(reordered_line)
    idex += 1

wrt.write('}')

file.close()
wrt.close()
