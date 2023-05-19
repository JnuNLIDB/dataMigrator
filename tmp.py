import orjson as json

print("Parsing Json")
data: list = json.loads(open('report_2020.json', encoding='utf-8').read())
print("Done")
for n in data:
    if "Source" not in n or "Id_" not in n["Source"][0]:
        print(n["Source"])
