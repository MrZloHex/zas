.DEF	A 0d17
.DEF	B 0d13

SECTION TEXT

	MIH 0x80
	MIL 0x10
	MIA A
	MIB B
	MID 1
	MIE 1
	MAC
	LSP
loop:	ADC
	PUA
	MEA
	ADD
	CPB
	MAE
	POA
	MIH loop%H
	MIL loop%L
	JRZ
	HLT

END