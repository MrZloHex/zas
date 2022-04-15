.DEF	A 0d2
.DEF	B 0d4
.DEF	C 0d8

SECTION TEXT
		MIH _start%H
		MIL _start%L
		JMP

END

.INCLUDE	multiply.asm

SECTION TEXT

_start:		MIH 0x81
		MIL 0x00
		LSP
		MIA A
		MIB B
		CALL(multiply, _back1)
_back1:		MIB C
		CALL(multiply, _back2)
_back2:		HLT


END
