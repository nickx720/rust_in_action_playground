Coding Challenge #50 - Build Your Own Xargs

This challenge is to build your own version of the Unix command line tool xargs!

The Unix command line tools are a great metaphor for good software engineering and they follow the Unix Philosophies of:

    Writing simple parts connected by clean interfaces - each tool does just one thing and provides a simple CLI that handles text input from either files or file streams.

    Design programs to be connected to other programs - each tool can be easily connected to other tools, via files and streams, to create incredibly powerful compositions.

Following these philosophies has made the Unix command line tools some of the most widely used software engineering tools which can be chained together to create far more complex and powerful set of tools that you’d expect.

Xargs epitomises the philosophy by providing a tool to allow us to connect together programs effectively, using the output of one to configure the behaviour of the next.
If You Enjoy Coding Challenges Here Are Four Ways You Can Help Support It


The Challenge - Building You Own Xargs

This challenge is to build your own version of xargs. As always with command line tools a great way to find out what the tool does and how to use it is to use man:

NAME
     xargs – construct argument list(s) and execute utility

SYNOPSIS
     xargs [-0oprt] [-E eofstr] [-I replstr [-R replacements]
           [-S replsize]] [-J replstr] [-L number] [-n number [-x]]
           [-P maxprocs] [-s size] [utility [argument ...]]

DESCRIPTION
     The xargs utility reads space, tab, newline and end-of-file
     delimited strings from the standard input and executes utility
     with the strings as arguments.

     Any arguments specified on the command line are given to
     utility upon each invocation, followed by some number of the
     arguments read from the standard input of xargs.  This is
     repeated until standard input is exhausted.

You can read about how useful xargs can be in my Developing Skills newsletter article that explains how I used it to build a simple load testing tool for a RESTful API.
Step Zero

In this introductory step you’re going to set your environment up ready to begin developing and testing your solution.

I’ll leave you to setup your IDE / editor of choice and programming language of choice.
Step 1

In this step your goal is to build the command ccxargs that will take a whitespace separated set of strings from standard in and convert them into command line arguments that can be passed to a command (referred to as utility in the man page quoted above).

You can test your code using, this command below to create three text files we can use for testing:

% for i in {1..3}; do echo "This is file ${i}" > test-${i}.txt; done;

Then in the same directory we can use ls to create a whitespace separated list of files and pipe that into our ccxargs program which we will tell to run the command cat with each of the items in the list as the argument to cat:

% ls | ccxargs cat
This is file 1
This is file 2
This is file 3

This is the equivalent of having done:

% cat test-1.txt test-2.txt test-3.txt
This is file 1
This is file 2
This is file 3

Bonus points if you use your own version of cat from the build your own cat Coding Challenge.
Step 2

In this step your goal is to support additional command line arguments being passed to the utility command. We can test this using sed like so:

% ls | ccxargs sed s/file/test/g
This is test 1
This is test 2
This is test 3

But don’t stop there, ensure you test with 2+ arguments to the command too.
Step 3

In this step your goal is to support the -n option, which does this (from the man page):

 -n number, --max-args=number
             Set the maximum number of arguments taken from standard
             input for each invocation of utility.  An invocation of
             utility will use less than number standard input arguments
             if the number of bytes accumulated (see the -s option)
             exceeds the specified size or there are fewer than number
             arguments remaining for the last invocation of utility.  The
             current default value for number is 5000.

You can then test this like so:

% ls | ccxargs -n 1 cat
This is file 1
This is file 2
This is file 3

This time your code should have invoked cat three times, one after the other.
Step 4

In this step your goal is to support the -P option, which does this (from the man page):

     -P maxprocs, --max-procs=maxprocs
             Parallel mode: run at most maxprocs invocations of utility
             at once.  If maxprocs is set to 0, xargs will run as many
             processes as possible.

To test this I suggest creating a text file with a list of URLs in it, say urls.txt, then use ccxargs to invoke curl to download the pages.

% cat urls.txt | ccxargs -n 1 -P 1 curl

Which will dump the content of the websites you hit to your console. You might like to time it.

Then run the test again with a higher value of P to see the overall time reduced as the curl requests are sent concurrently.

% cat urls.txt | ccxargs -n 1 -P 10 curl
