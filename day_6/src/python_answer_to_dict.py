import json

if __name__ == "__main__":
    txt_lines = []
    with open("./small_answer.txt") as fl:
        txt_lines = fl.readlines()
    to_dict = {}
    for line in txt_lines:
        split = line.replace("\n", "").split(":")
        numbs = split[1].split(",")
        to_dict[split[0]] = {}
        for num in numbs:
            if num.strip() in to_dict[split[0]]:
                to_dict[split[0]][num.strip()] += 1
            else:
                to_dict[split[0]][num.strip()] = 1

    with open("./json_days_dict", 'w') as fl:
        json.dump(to_dict, fl)