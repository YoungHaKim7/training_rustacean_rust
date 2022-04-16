# 변수 선언
# 빌드 변수 
CC=g++

# 옵션 변수
CFLAGS= -o

# 오브젝트 변수 
OBJS=main.o function.o

# 타겟 변수 
TARGET=main

# 변수를 사용하려면 $ 를 사용한다. 
$(TARGET): $(OBJS)

# $@ 은 현재 Target 이름을 뜻한다.
	$(CC) $(CFLAGS) $@ $(OBJS)

# 오브젝트 생성 
main.o : function.h main.cpp
function.o: function.h function.cpp

