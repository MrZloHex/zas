.INCLUDE	std.asm

SECTION TEXT
		LEA(_start)
		JMP
END

.INCLUDE	multiply.asm

SECTION TEXT

_start:		INIT_STACK
		LEA(num_A)
		MMA
		LEA(num_B)
		MMB
		CALL(multiply, _back1)
_back1:		LEA(num_C)
		MMB
		CALL(multiply, _back2)
_back2:		LEA(result)
		MAM
		MAB
		MAC
		MAD
		MAE
		MAH
		MAL
		HLT

END

SECTION BSS

result:		1

END

SECTION DATA

num_A:		0x02 0x05
num_B:		0x04
num_C:		0x08

END
