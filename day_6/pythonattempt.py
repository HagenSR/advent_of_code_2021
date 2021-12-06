
if __name__ == "__main__":
    fish = {}
    with open("./src/data.txt") as fl:
        line = fl.readline()
        strs = line.split(",")
        for str in strs:
            if int(str) not in fish:
                fish[int(str)] = 1
            else:
                fish[int(str)] += 1
    for i in range(0,256):
        new_dict = {}
        for key in fish:
            new_dict_key = key - 1
            if new_dict_key == -1:
                if 6 in new_dict:
                    new_dict[6] += fish[key]
                else:
                    new_dict[6] = fish[key]
                if 8 in new_dict:
                    new_dict[8] += fish[key]
                else:
                    new_dict[8] = fish[key]
            else:
                if new_dict_key in new_dict:
                    new_dict[new_dict_key] += fish[key]
                else:
                    new_dict[new_dict_key] = fish[key]
        fish = new_dict
        if i == 79 or i == 255:
            print("There are {} fish after {} days".format(sum(fish.values()), i + 1))
        
