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