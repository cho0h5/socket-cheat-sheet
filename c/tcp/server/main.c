#include <stdio.h>
#include <sys/socket.h>
#include <netinet/in.h>

int main() {
	int sockfd = socket(AF_INET, SOCK_STREAM, 0);

	struct sockaddr_in server_addr;
	memset(&server_addr, 0, sizeof(server_addr));
	server_addr.sin_family = AF_INET;
	server_addr.sin_port = htons(8090);
	server_addr.sin_addr.s_addr = INADDR_ANY;

	bind(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr));

	listen(sockfd, 1);
	printf("0\n");

	int connfd = accept(sockfd, (struct sockaddr*)NULL, NULL);
	printf("1\n");

	char buf[] = "hello, client";

	write(connfd, buf, 14);
	printf("send to client: %s\n", buf);

	close(connfd);
	close(sockfd);

	return 0;
}
