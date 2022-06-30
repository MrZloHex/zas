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
m4_define(`SAVE_REG', `PUB
		PUC
		PUD
		PUE')m4_dnl
m4_define(`LOAD_REG', `POE
		POD
		POC
		POB')m4_dnl
m4_define(`LL', `MI$2 $1%H
		MI$3 $1%L')m4_dnl
m4_define(`LADR', `MI$3 $1
		MI$4 $2')m4_dnl
m4_define(`LEIA', `MIH $1
		MIL $2 ')m4_dnl
m4_define(`NULL', `0')m4_dnl
