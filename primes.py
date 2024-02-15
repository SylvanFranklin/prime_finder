# Start with a set W={1} and length=1 representing wheel 0, and prime p=2.
# As long as p2 <= N, do the following
#     if length < N then
#         extend W by repeatedly getting successive members w of W starting with 1 and inserting length+w into W as long as it doesn't exceed p*length or N;
#         increase length to the minimum of p*length and N.
#     repeatedly delete p times each member of W by first finding the largest <= length and then working backwards.
#     note the prime p, then set p to the next member of W after 1 (or 3 if p was 2).
# if length < N then extend W to N by repeatedly getting successive members w of W starting with 1 and inserting length+w into W as long as it doesn't exceed N;
# On termination, the rest of the primes up to N are the members of W after 1.

def pritchard(n):
    w = [1]
    length = 1
    primes = [2, 3]
    p = 2

    while p ** 2 <= n:
        # make an initial wheel
        if length < n:
            for i in w:
                j = i + length 
                if j < p * length and p < n:
                    w += j

            length = min(p*length, n)
        # remove the duplicates
        for i in range(len(wheel)):
            if wheel[i] % p == 0:
                wheel.pop(i)

        # go to the next prime
        primes.append(p)
        p = w[1]

    return primes
