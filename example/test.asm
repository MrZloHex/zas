.INCLUDE	std.asm

.DEF	A 0d2
.DEF	B 0d4
.DEF	C 0d8

SECTION TEXT
		LEA(_start)
		JMP
END

.INCLUDE	multiply.asm

SECTION TEXT

_start:		INIT_STACK
		MIA A
		MIB B
		CALL(multiply, _back1)
_back1:		MIB C
		CALL(multiply, _back2)
_back2:		HLT


END

