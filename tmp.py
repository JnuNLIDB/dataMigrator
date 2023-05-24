import os
import logging

import orjson as json

logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)

for file in os.listdir("data"):
    new_data = []

    logger.info("Open file: ", file)
    j = open("data/" + file, encoding='utf-8').read()
    j = "[" + j.replace("\n", ",").rstrip(",") + "]" if j.count("\n") > 0 else j
    logger.info("Load json")
    data: list = json.loads(j)
    for i, n in enumerate(data):
        if "Source" not in n or any(("Id_" not in k for k in n["Source"])):
            continue
        if "Opinion" not in n["People"] or type(n["People"]["Opinion"]) is dict:
            n["People"]["Opinion"] = [n["People"]["Opinion"]]
        new_data.append(n)
    logger.info("Total: ", len(new_data))

    new_file_name = file.replace(".json", "_new.json")

    with open(new_file_name, "wb") as f:
        f.write(json.dumps(new_data))
#
# with open('report_2021_new.json', encoding='utf-8') as f:
#     string = f.read()
#     # Print the string in column 358500 to 358600
#     print(string[28216700:28216800])
if __name__ == '__main__':
    pass