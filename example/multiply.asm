.DEF	A 0d2
.DEF	B 0d4
.DEF	C 0d8


.DEF	CALL
	; $1 - label to calling function
	; $2 - label where to return after return
		MIH $2%H
		MIL $2%L
		PUL
		PUH
		MIH $1%H
		MIL $1%L
		JMP
.ENDDEF

.DEF	RET
		POH
		POL
		JMP
.ENDDEF


SECTION TEXT

		MIH 0x81
		MIL 0x00
		LSP
		MIA A
		MIB B
		CALL(multiply, _back1)
_back1:		MIB C
		CALL(multiply, _back2)
_back2:		HLT


multiply:	MID 1
		MAC
		MIE 1
loop:		ADC
		PUA
		MEA
		ADD
		CPB
		MAE
		POA
		MIH loop%H
		MIL loop%L
		JRZ
		RET

END