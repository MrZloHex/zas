label:	MIH 80
	MIL 0xFF
	MIA 0o32
	MIB 0b110
	LSP

	ADB
	MIH label%H
jump:	MIL label%L
	; COMMENT
	JMP
	HLT

