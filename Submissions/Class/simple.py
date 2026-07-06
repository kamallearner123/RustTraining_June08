name = input("Please enter your name:") 
age = int(input("Please enter your age:"))

print("Your name is ", name)

price = 100
charge = 0
# No ticket, half ticket, full ticket, Senior
# < 5 yes: No ticket
# 6 to 10 : half
# 11 to 60: full ticket
# >60: senior

# if condition, loop, break
if age < 6:
    charge = 0
elif age < 10: # else if => elif
    charge = price/2
elif age < 60:
    charge = price
else:
    charge = price*0.7

print("Please pay: ", charge)
