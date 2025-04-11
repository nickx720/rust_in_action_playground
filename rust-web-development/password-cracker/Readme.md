Step Zero

In many programming languages we index arrays from zero onwards. Coding Challenges is the same, we start with Step 0. It’s the step where you setup your IDE / editor of choice and programming language of choice.

Depending on whether you’re going to aim for more of a John the Ripper or a CrackStation you might pick a stack like C, C++, Rust or Go versus a stack like PHP, Python or JavaScript. The choice is yours!
Step 1

In this step your goal is to implement the MD5 hash function. By doing so you will have an awareness of how password hashes are generated. Wikipedia has an explanation of the MD5 algorithm.

You can test your implementation against the implementation in your programming languages standard library. In the event that it doesn’t have support you can compare to this Python that you could run locally or on one of the online IDEs.

from hashlib import md5

print(md5(b'password').hexdigest())

Step 2

In this step your goal is to crack an MD5 password by brute force. To do that you’ll want to generate all the possible permutations of valid password characters up to a predefined length, then hash them and compare to a pre-determined hashed password.

As a test case try some four letter passwords and brute force them. Here’s a couple you could try:

7a95bf926a0333f57705aeac07a362a2
08054846bbc9933fd0395f8be516a9f9

This is the equivalent of incremental mode in John the Ripper.
Step 3

In this step your goal is to use a word list to speed up the attack. Instead of generated every single possible permutation of letters we’ll use a word list of common passwords. You can get one such list from CrackStation here. Grab the Smaller Wordlist for now.

Adapt your program to allow the user to specify whether to brute force or use a word list, allowing them to specify the path to the word list. See how quickly you can crack this hash: 2bdb742fc3d075ec6b73ea414f27819a
Step 4

In this step your goal is to build your own rainbow table. A rainbow table is a set of pre-computed hashes. For this process you can read in the word list and/or generate all the possible permutations of valid password characters up to a set length, then compute the hash for them. Save that hash and the input ‘password’ used to generate the hash to a file.
Step 5

In this step your goal is to crack an password using the rainbow table. Simply put, your code will now read in the pre-computed rainbow table and look up the hash to ‘crack’ in it.
Step 6 (Bonus)

In this step your goal is to add support for other common cryptographic hashing functions. After than read up on and learn about salting and key derivation functions (KDF).
Going Further

You can take this challenge further by optimising your solution to use multiple threads to compute (or look up) the hashes in parallel. You can go even further down that road by looking at how GPUs are used to accelerate password cracking.
