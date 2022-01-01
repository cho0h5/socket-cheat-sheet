#include <stdio.h>
#include <sys/socket.h>
#include <netinet/in.h>

int main() {
	int sockfd = socket(AF_INET, SOCK_STREAM, 0);

	struct sockaddr_in server_addr;
	memset(&server_addr, 0, sizeof(server_addr));
	server_addr.sin_family = AF_INET;
	server_addr.sin_port = htons(8090);
	server_addr.sin_addr.s_addr = inet_addr("127.0.0.1");

	if( -1 == connect(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr))) {
		printf("connect failed\n");
		exit(1);
	}

	char buf[20];

	read(sockfd, buf, sizeof(buf));
	printf("from serer: %s\n", buf);

	close(sockfd);

	return 0;
}
