m4_define(`A', `0d2')m4_dnl
m4_define(`B', `0d4')m4_dnl
m4_define(`C', `0d8')m4_dnl
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
