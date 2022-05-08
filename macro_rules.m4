m4_define(`LEA', `MIH $1%H
		MIL $1%L')m4_dnl
m4_define(`CALL', `MIH $2%H
		MIL $2%L
		PUL
		PUH
		MIH $1%H
		MIL $1%L
		JMP')m4_dnl
m4_define(`RET', `POH
		POL
		JMP')m4_dnl
m4_define(`INIT_STACK', `MIH 0x90
		MIL 0x00
		LSP')m4_dnl
m4_define(`MALLOC', `')m4_dnl
