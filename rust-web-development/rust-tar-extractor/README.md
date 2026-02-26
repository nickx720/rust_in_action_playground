The Challenge - Building Your Own Tar Tool.

In this Coding Challenge we’re going to build a version of the tar tool that can handle creating, listing and unpacking tarballs in the Unix Standard TAR (UStar) format. The format is relatively simple and you can find a good explanation on Wikipedia here.

The functional requirements for tar are concisely described by it’s man page - give it a go in your local terminal now:

man tar

# Step Zero

Like all good arrays we’re zero indexed! In this step you’re going to set your environment up ready to begin developing and testing your solution.

I’ll leave you to setup your IDE / editor of choice and programming language of choice. Tar is a command line tool so I’d suggest picking a suitable language.

After that please use the tar on your computer to create a simple test tarball by running the following commands:

echo "File 1 contents" >> file1.txt
echo "File 2 contents" >> file2.txt
echo "File 3 contents" >> file3.txt
tar -cf files.tar file1.txt file2.txt file3.txt

If you’re on Windows you could use WSL or run a Linux container.

After running these commands you should have a tarball we can use for testing called files.tar.
# Step 1

In this step your goal is to list the files in an archive. If we use tar that looks something like this when reading from standard in:

% cat files.tar | tar -t
file1.txt
file2.txt
file3.txt

To do that you’re going to have to read the tarball and parse the file headers. To do that you need to understand the tarball file format.

N.B. many programming languages have libraries that will read and white tarballs, unless you’re an absolute novice programmer I strongly recommend learning to read and implement that reading and writing of the tarball yourself. You’ll learn far more from the exercise and hopefully that’s what you’re here for. 😀

A tarball consists of a series of file objects. Each object is made up of a 512-byte header, followed by as many 512-byte blocks as are needed to contain the file contents, rounded up to the nearest full block. After the final file object there is at least two consecutive 512-bytes blocks of zero-filled records. Full details can be found in Wikipedia.

You can also use xxd to explore the tarball itself: here’s the first header for mine:

 % xxd -l 512 files.tar
00000000: 6669 6c65 312e 7478 7400 0000 0000 0000  file1.txt.......
00000010: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000020: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000030: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000040: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000050: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000060: 0000 0000 3030 3036 3434 2000 3030 3037  ....000644 .0007
00000070: 3635 2000 3030 3030 3234 2000 3030 3030  65 .000024 .0000
00000080: 3030 3030 3032 3020 3134 3537 3730 3431  0000020 14577041
00000090: 3035 3120 3031 3434 3435 0020 3000 0000  051 014445. 0...
000000a0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
000000b0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
000000c0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
000000d0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
000000e0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
000000f0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000100: 0075 7374 6172 0030 306a 6f68 6e63 7269  .ustar.00johncri
00000110: 636b 6574 7400 0000 0000 0000 0000 0000  ckett...........
00000120: 0000 0000 0000 0000 0073 7461 6666 0000  .........staff..
00000130: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000140: 0000 0000 0000 0000 0030 3030 3030 3020  .........000000
00000150: 0030 3030 3030 3020 0000 0000 0000 0000  .000000 ........
00000160: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000170: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000180: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000190: 0000 0000 0000 0000 0000 0000 0000 0000  ................
000001a0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
000001b0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
000001c0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
000001d0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
000001e0: 0000 0000 0000 0000 0000 0000 0000 0000  ................
000001f0: 0000 0000 0000 0000 0000 0000 0000 0000  ................

When you have completed this step, it should look like:

% cat files.tar | cctar -t
file1.txt
file2.txt
file3.txt

You could optionally also support reading the file directly:

% cctar -tf files.tar
file1.txt
file2.txt
file3.txt

# Step 2 Done

In this step your goal is to extract the files from an archive, creating the equivalent of the tar command:

% tar -xf files.tar

Be sure to check that you’re writing just the content of the original file to the extracted file. You can check by ensuring that the original file and the extracted ones are the same size and/or using diff.

Step 3

In this step your goal is to create a tarball. In other words write the code to do the equivalent of the command we ran earlier:

% tar -cf files.tar file1.txt file2.txt file3.txt

When you’ve done that you can test the tarball you create is correct by:

% cctar -cf filescc.tar file1.txt file2.txt file3.txt
% diff files.tar filescc.tar

If you’re writing the correct format there should be no difference.
Going Further

You can take this challenge further by adding support for different tarball formats and the various extensions such as gzipping (and unzipping) the tarballs. Check the man page for tar for more ideas.
2 Other Ways I Can Help You:

    I write another FREE newsletter Developing Skills that helps you level up the other skills you need to be a great software developer.

    I have some courses available:

        Build Your Own Redis Server (Python Edition) which guides you through solving the Redis Coding Challenge in Python.

        Build Your Own Shell (Go Edition) which guides you through solving the Shell Coding Challenge in Go.

Share Your Solutions!

If you think your solution is an example of the developers can learn from please share it, put it on GitHub, GitLab or elsewhere. Then let me know via Twitter or LinkedIn or just post about it there and tag me.
Request for Feedback

I’m writing these challenges to help you develop your skills as a software engineer based on how I’ve approached my own personal learning and development. What works for me, might not be the best way for you - so if you have suggestions for how I can make these challenges more useful to you and others, please get in touch and let me know. All feedback greatly appreciated.

You can reach me on Twitter, LinkedIn or through SubStack

Thanks and happy coding!

John

You're currently a free subscriber to Coding Challenges. For the full experience, upgrade your subscription.

Upgrade to paid

Like

Comment

Restack



© 2024 John Crickett
548 Market Street PMB 72296, San Francisco, CA 94104
Unsubscribe

Get the appStart writing
