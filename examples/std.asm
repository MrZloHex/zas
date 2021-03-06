.DEF	LEA
	; Load Effective Address
		MIH $1%H
		MIL $1%L
.ENDDEF

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

.DEF 	SAVE_REG
		PUB
		PUC
		PUD
		PUE
.ENDDEF

.DEF 	LOAD_REG
		POE
		POD
		POC
		POB
.ENDDEF

	; LL - Load Label
.DEF 	LL
	; $1 - Label
	; $2 - High Register
	; $3 - Low  Register
		MI$2 $1%H
		MI$3 $1%L
.ENDDEF

	; LADR - Load Address
.DEF 	LADR
	; $1 - High Address
	; $2 - Low  Address
	; $3 - High Register
	; $4 - Low  Register
		MI$3 $1
		MI$4 $2
.ENDDEF

	; LADR - Load Effective Imm Address
.DEF 	LEIA
	; $1 - High Address
	; $2 - Low  Address
		MIH $1
		MIL $2 
.ENDDEF

.DEF 	NULL 	0


SECTION TEXT
	; A - H reg
	; B - L reg
inc_address:	MAH
		MBL
		MIA 1
		ADL
		MAL
		PUH
		PUL
		LEA(_exit_inc_addr)
		JRZ
		
		POL
		POH
		MIA 1
		ADH
		MAH
		PUH
		PUL

_exit_inc_addr:	POB
		POA
		RET


END
