mkdir -p target;
gcc -std=gnu11 -Wall -c src/elev.c -o target/elev.o;
gcc -std=gnu11 -Wall -c src/io.c -o target/io.o;
dmd -w -g -oftarget/ElevatorServer src/elevatorserver.d target/elev.o target/io.o -L-lcomedi -L-lm
