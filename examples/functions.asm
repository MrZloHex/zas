SECTION TEXT

		MIH 0xEF
		MIL 0xFF
		LSP

		MIA str%H
		MIB str%L

qwe:		PUC
		PUD
		PUE
		MAH
		MBL

		MIE 0

		MMA
		MID 0
		CPD
		PUH
		PUL
		LEA(_exit)
		JSZ

		POL
		POH
		PUA
		MHA
		MLB
		CALL(inc_address, _h1)
_h1:		MAH
		MBL
		POA

		MID 0x29 + 1
		CPD
		LEA(_exit)
		JSC

		MID 0x3A
		CPD
		LEA(hex_letter)
		JRC

		MID 0x30
		SUD
		LEA(end_loop)
		JMP

hex_letter:	MID 0x41
		CPD
		LEA(_exit)
		JSC

		MID 0x47
		CPD
		LEA(_exit)
		JRC

		MID 0x41
		SUD
		MID 10
		ADD

end_loop:	MAE

		

_exit:	HLT
END

.INCLUDE 	std.asm
SECTION DATA

str:  0x46 0x38 NULL

END
