#take in our path from lkh, and translate to our character values
def main():
    file_path = 'challenge_body.txt'  
    with open(file_path, 'r') as f:
        header = f.readline().strip().split()
        node_names = f.readline().strip().split()

    print(node_names)
    temp = ""
    with open("bricks.txt", 'r') as f:
        for index, line in enumerate(f):
            #print(line, index)
            if index <= 5:
                pass
            else:
                print(index, line)
                temp += (node_names[int(line)-1])
                print(temp)
    print(temp)

if __name__ == "__main__":
    main()

