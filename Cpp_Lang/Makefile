# 변수 선언
CC=g++
CFLAGS= -o
OBJS=main.o function_test.o
TARGET=main

# 실행 파일 만드는 타겟팅 
all: $(TARGET)

# make clean의 명령어를 수행.
clean:
	rm -rf *.o
	rm -rf $(TARGET)


$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) $@ $(OBJS)

# 의존성 오브젝트 생성 
main.o : function_test.h main.cpp
function_test.o: function_test.h function_test.cpp

