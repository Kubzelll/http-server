//
// Created by kubzel on 3/11/24.
//

#include "http_tcp_server.h"
#include <sstream>
#include <unistd.h>
#include <iostream>

namespace
{
    void log(const std::string &message)
    {
        std::cout << message << std::endl;
    }
    void  exitWithError(const std::string &errorMessage)
    {
        log("[!] ERROR: " + errorMessage);
        exit(1);
    }
}
namespace http
{

    TcpServer::TcpServer(std::string ip_address, int port)
     : m_ip_address(ip_address), m_port(port), m_socket(),
       m_new_socket(), m_incomingMessage(), m_socketAddress(),
       m_socketAddress_len(sizeof(m_socketAddress)),
       m_serverMessage(buildResponse())
    {
        startServer()
    }

    TcpServer::~TcpServer()
    {

    }

    int TcpServer::startServer() {
        return 0;
    }

    void TcpServer::closeServer() {

    }
    std::string TcpServer::buildResponse()
    {
        std::string htmlFile = "<!DOCTYPE html><html lang=\"en\"><body><h1> HOME </h1><p> Hello from your Server :) </p></body></html>";
        std::ostringstream ss;
        ss << "HTTP/1.1 200 OK\nContent-Type: text/html\nContent-Length: " << htmlFile.size() << "\n\n"
           << htmlFile;

        return ss.str();
    }


}