#include <stdio.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
#include <string.h>
#include <arpa/inet.h>
#include <time.h>

int main() {
	int sockfd = socket(AF_INET, SOCK_STREAM, 0);

	struct sockaddr_in addr;
	memset(&addr, 0, sizeof(addr));
	addr.sin_family = AF_INET;
	addr.sin_port = htons(8080);
	addr.sin_addr.s_addr = inet_addr("127.0.0.1");

	connect(sockfd, (struct sockaddr*)&addr, sizeof(addr));

	char buffer[1024];

	clock_t start = clock();
	write(sockfd, "hello server", 13);
	read(sockfd, buffer, sizeof(buffer));
	clock_t duration = clock() - start;

	printf("%fms\n", (float)duration / CLOCKS_PER_SEC * 1000);
	printf("return: %s\n", buffer);

	close(sockfd);

	return 0;
}
