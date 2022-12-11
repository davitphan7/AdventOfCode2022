import os

if __name__ == "__main__":


    dirname = os.path.dirname(__file__)
    filename = os.path.join(dirname, 'input.txt')


    with open(filename,'r') as fin:
        lines = fin.readlines()

        ElvesList = list() 
        currentElfSum = 0

        for i in lines:
            if i == "\n":
                ElvesList.append(currentElfSum)
                currentElfSum = 0
            else:
                currentElfSum += int(i)
        ElvesList.append(currentElfSum)




    ElvesList.sort(reverse=True)
    print(ElvesList)
    print(ElvesList[0] + ElvesList[1] + ElvesList[2])

