import orjson as json

print("Parsing Json")
data: list = json.loads(open('report_2023.json', encoding='utf-8').read())
k = 0
new_data = []
for i, n in enumerate(data):
    if "Source" not in n or any(("Id_" not in k for k in n["Source"])):
        continue
    if "Opinion" not in n["People"] or type(n["People"]["Opinion"]) is dict:
        n["People"]["Opinion"] = [n["People"]["Opinion"]]
    new_data.append(n)

with open("report_2023_new.json", "wb") as f:
    f.write(json.dumps(new_data))
#
# with open('report_2021_new.json', encoding='utf-8') as f:
#     string = f.read()
#     # Print the string in column 358500 to 358600
#     print(string[28216700:28216800])
if __name__ == '__main__':
    pass