_start:		MIH 0x81
		MIL 0x00
		LSP
	; MAIN SECTION
	; READ KEYBOARD MAYBE ??
		MIH _here1%H
		MIL _here1%L
		PUL
		PUH
		MIH keyboard_input%H
		MIL keyboard_input%L
		JMP
	; IN REG A IS CODE OF SYMBOL
_here1:		MIA 0x69
		HLT



keyboard_input: MIH 0x8F ; MEMMAPED KEAYBOARD
		MIL 0x10
		
		POH
		POL
		JMP
