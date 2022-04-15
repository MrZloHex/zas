

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