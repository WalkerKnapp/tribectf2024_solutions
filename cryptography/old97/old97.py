#Old 97 solution, pretty directly implemented
f = open("ciphers.txt", "r")
lines = f.readlines()
chars = ""
x = 64 #computed directly

for line in lines:
    c2, c1 = map(int, line.strip().split(", ")) #this order is key!
    s = (c1 ** x) % 97
    s_inv = (s**95) % 97 #the magic 95 is simply phi(97)-1
    m = (c2 * s_inv) % 97
    chars += (chr(m+30)) #does the inverse mapping
print(chars)

