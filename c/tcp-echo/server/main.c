#include <stdio.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <string.h>
#include <unistd.h>

int main() {
	int sockfd = socket(AF_INET, SOCK_STREAM, 0);

	struct sockaddr_in addr;
	memset(&addr, 0, sizeof(addr));
	addr.sin_family = AF_INET;
	addr.sin_port = htons(8080);
	addr.sin_addr.s_addr = INADDR_ANY;

	bind(sockfd, (struct sockaddr*)&addr, sizeof(addr));

	listen(sockfd, 1);


	char buffer[1024];

	for(;;) {
		int connfd = accept(sockfd, (struct sockaddr*)NULL, NULL);
		read(connfd, buffer, sizeof(buffer));
		write(connfd, buffer, sizeof(buffer));
		close(connfd);
	}

	close(sockfd);

	return 0;
}
