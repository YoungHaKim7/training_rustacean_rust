# <Target>: <Dependencies>
#		<Recipe>
# Target은 대상 : Dependencies는 파일이 있는 지 목록을 체크하는 것, 없으면 Target을 찾습니다.
# Recipe 는 Target을 만들기 위한 명령어 .

# main.exe 를 만들기 위해서 main.o 와 function.o 를 체크한다.
main.exe: main.o function.o

# main.exe 를 만드는 명령어
	g++ -o main main.o function.o

# main.o 를 만들기 위해서 function.h와 main.cpp 를 체크한다. 
main.o : function.h main.cpp

# main.o 를 만드는 명령어 
	g++ -c -o main.o main.cpp

# function.o 를 만들기 위해서 function.h와 function.cpp 를 체크한다.
function.o: function.h function.cpp

# function.o 를 만드는 명령어 
	g++ -c -o function.o function.cpp
