# how to run R -f whales.r
5 + 5

whales = c(74, 122, 235, 111, 292, 111, 211, 133, 156, 79)

sum(whales)

length(whales)

sum(whales)/length(whales)

mean(whales)

sort(whales)

min(whales)

max(whales)

range(whales)

# ?
diff(whales)

cumsum(whales)

whales.fla = c(89, 254, 306, 292, 274, 233, 294, 204, 204, 90)

whales + whales.fla

whales - whales.fla

whales.fla - mean(whales)



x = c(2,3,5,7,11)
xbar = mean(x)
x - xbar
(x - xbar) ^ 2
sum((x - xbar) ^ 2)

n = length(x)
n
sum((x - xbar) ^ 2) / (n - 1)