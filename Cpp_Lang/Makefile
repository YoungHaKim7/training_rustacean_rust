main.out : main.o function_test.o

	g++ -o main.out main.o function_test.o

main.o : function_test.h main.cpp

function_test.o: function_test.h function_test.cpp

# make clean의 명령어를 수행.
clean:
	rm -rf *.o
	rm -rf $(TARGET)
