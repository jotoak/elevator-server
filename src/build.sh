mkdir -p ../target
gcc -std=gnu11 -Wall -c elev.c -o ../target/elev.o;
gcc -std=gnu11 -Wall -c io.c -o ../target/io.o;
dmd -w -g -of../target/ElevatorServer elevatorserver.d ../target/elev.o ../target/io.o -L-lcomedi -L-lm