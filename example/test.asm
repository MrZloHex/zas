.DEF	MACRO HLT
.DEF	my_var 0x80

	MIH my_var
	MIL 0xFF
	MIH jump%H
	MIL jump%L
label:	MIA 0o32
	MIB 0b110
	LSP

	ADB
jump:	MIH label%H
	MIL label%L
	; COMMENT
	JMP
	MACRO
