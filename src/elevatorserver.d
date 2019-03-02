import  std.conv,
        std.datetime,
        std.getopt,
        std.socket,
        std.stdio;

extern(C){
    void elev_init();

    void elev_set_motor_direction(int dirn);
    void elev_set_button_lamp(int button, int floor, int value);
    void elev_set_floor_indicator(int floor);
    void elev_set_door_open_lamp(int value);
    void elev_set_stop_lamp(int value);

    int elev_get_button_signal(int button, int floor);
    int elev_get_floor_sensor_signal();
    int elev_get_stop_signal();
    int elev_get_obstruction_signal();
}

void main(){

    elev_init();
    
    Socket acceptSock = new TcpSocket();

    acceptSock.setOption(SocketOptionLevel.SOCKET, SocketOption.REUSEADDR, 1);
    acceptSock.bind(new InternetAddress(15657));
    acceptSock.listen(1);

    ubyte[4] buf;

    writeln("Elevator server started");
    
    while(true){
        auto sock = acceptSock.accept();
        writeln("[", Clock.currTime, "]: Connected to ", sock.remoteAddress);
        while(sock.isAlive){
            buf = 0;
            auto n = sock.receive(buf);

            if(n <= 0){
                elev_set_motor_direction(0);
                sock.shutdown(SocketShutdown.BOTH);
                sock.close();
                writeln("[", Clock.currTime, "]: Disconnected");
            } else {
                switch(buf[0]){
                case 1:
                    elev_set_motor_direction(
                        (buf[1] == 0)   ? 0  :
                        (buf[1] < 128)  ? 1  :
                                          -1
                    );
                    break;
                case 2:
                    elev_set_button_lamp(buf[1], buf[2], buf[3]);
                    break;
                case 3:
                    elev_set_floor_indicator(buf[1]);
                    break;
                case 4:
                    elev_set_door_open_lamp(buf[1]);
                    break;
                case 5:
                    elev_set_stop_lamp(buf[1]);
                    break;

                case 6:
                    buf[1..$] = [elev_get_button_signal(buf[1], buf[2]).to!ubyte, 0, 0];
                    sock.send(buf);
                    break;
                case 7:
                    buf[1..$] = [elev_get_floor_sensor_signal().to!ubyte, 0, 0];
                    sock.send(buf);
                    break;
                case 8:
                    buf[1..$] = [elev_get_stop_signal().to!ubyte, 0, 0];
                    sock.send(buf);
                    break;
                case 9:
                    buf[1..$] = [elev_get_obstruction_signal().to!ubyte, 0, 0];
                    sock.send(buf);
                    break;
                    
                default:
                    break;
                }
            }
        }
    }
}