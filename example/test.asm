.DEF	my_var 0x80
.DEF	MACRO HLT

.DEF	FUNC
		MIA $1
		MIB $2
.ENDDEF

SECTION TEXT

		MAC my_var
		MIL 0xFF
		MIH jump%H
		MIL jump%L
label:		MIA 0o32
		MIB 0b110
		LSP
		FUNC(0xBE, 0xEF)
END


SECTION TEXT

		ADB
jump:		MIH label%H
		MIL label%L
	; COMMENT
		JMP
		MACRO

END
