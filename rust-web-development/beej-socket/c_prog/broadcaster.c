#include <arpa/inet.h>
#include <errno.h>
#include <netdb.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

#define SERVERPORT 4950

int main(int argc, char *argv[]) {
  int sockfd;
  struct sockaddr_in their_addr;
  struct hostent *he;
  int numbytes;
  int broadcast = 1;

  if (argc != 3) {
    fprintf(stderr, "usage: broadcaster hostname message\n");
    exit(1);
  }

  if ((he = gethostbyname(argv[1])) == NULL) {
    perror("gethostbyname");
    exit(1);
  }

  if ((sockfd = socket(AF_INET, SOCK_DGRAM, 0)) == -1) {
    perror("socket");
    exit(1);
  }
}

