#!/usr/bin/env python3

from hashlib import pbkdf2_hmac
from secrets import randbits, token_bytes

coins = 100
flag = "tribectf{fake_flag_for_testing}"


def get_hash():
    key = get_random_bytes(8) + get_random_bytes(8)
    salt = get_random_bytes(16)
    hash = pbkdf2_hmac("md5", key, salt, 1000000)
    return hash


def get_random_bytes(n):
    return token_bytes(n)


def get_random():
    return get_random_bytes(16)


def menu():
    MENU = '''
Make a choice:

1. Buy flag (-200 coins)
2. Buy hint (-10 coins)
3. Play (+10/-50 coins)
4. Exit'''
    print("Coins: " + str(coins))
    print(MENU)


def main():
    print("Welcome to my casino stranger")
    global coins
    global flag
    while (coins > 0):
        try:
            menu()
            option = int(input("What would you like to do today? "))
        except:
            option = 5

        if option == 1:
            if coins >= 200:
                coins -= 200
                print("You have won!!!")
                print(flag)
                break
            else:
                print("You don't have enough coins!\n")
        elif option == 2:
            print("Imagine thinking I would help you. No\n")
            coins -= 10
        elif option == 3:
            bit = randbits(1)
            if bit == 1:
                print(get_random().hex())
            else:
                print(get_hash().hex())

            try:
                guess = int(input(
                    "Was the output a hash (0) or just random bytes (1), input a 0 or 1 respectively to choose "))
            except:
                guess = -1

            if guess == bit:
                print("Luck shines on you today\n")
                coins += 10
            else:
                print("Bad luck!\n")
                coins -= 50
        elif option == 4:
            print("Good Bye, you'll never free Griffin!!")
            break
        else:
            print("Please enter one of the valid number options\n")

    print("Now get out of my casino!")


if __name__ == "__main__":
    main()