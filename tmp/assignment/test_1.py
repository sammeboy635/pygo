# Parser: 102 microseconds || Tokenizer: 74 microseconds
test = 1 + 2 * (3 - 4) #[1,2,3,4,-,*,+]
test1 = 1 + 2 - 3 + 4 #[1,2,3,4,+,-,+]
test2 = 1 * (2 - 3) / 4 #[1,2,3,-,*,4,/]
test3 = 1 * (2 - 3) ** 4 #[1,2,3,-,4,**,*]
test4 = 0 - 1 * (2 - 3) ** 4 #[0,1,2,3,-,4,**,*,-]
test5 = 1 ** (2 - 3) * 4 #[1,2,3,-,**,4,*]
test6 = 0 - 1 ** (2 - 3) * 4 #[0,1,2,3,-,**,4,*,-]
test7 = 1 * 2 - 3 + 4 #[1,2,*,3,-,4,+]

test8 = test1 + test2

test9 = [1,2,3,4]


#print(test,test1,test2,test3,test4,test5,test6,test7,test8,test9)
#-1 4 -0.25 1 -1 4.0 -4.0 3 3.75 [1, 2, 3, 4]