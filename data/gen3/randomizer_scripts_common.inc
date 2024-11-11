Randomizer_Eventscript_Give_Pokemon::
	callnative GetItemBallIdAndAmountFromTemplate
	setvar VAR_TEMP_TRANSFERRED_SPECIES, VAR_RESULT
	givemon VAR_TEMP_TRANSFERRED_SPECIES, 5
	goto_if_eq VAR_RESULT, MON_GIVEN_TO_PARTY, Randomizer_Recieve_Pokemon_Party
	goto_if_eq VAR_RESULT, MON_GIVEN_TO_PC, Randomizer_Recieve_Pokemon_PC
	goto Common_EventScript_NoMoreRoomForPokemon
	end

Randomizer_Recieve_Pokemon_Party::
	call Randomizer_Recieve_Pokemon_Fanfare
	msgbox gText_NicknameThisPokemon, MSGBOX_YESNO
	call Common_EventScript_GetGiftMonPartySlot
	call Common_EventScript_NameReceivedPartyMon
	end

Randomizer_Recieve_Pokemon_PC::
	call Randomizer_Recieve_Pokemon_Fanfare
	msgbox gText_NicknameThisPokemon, MSGBOX_YESNO
	goto_if_eq VAR_RESULT, NO, Randomizer_Send_Pokemon_PC
	call Common_EventScript_NameReceivedBoxMon
	goto Randomizer_Send_Pokemon_PC
	end

Randomizer_Send_Pokemon_PC::
	call Common_EventScript_TransferredToPC
	end

Randomizer_Recieve_Pokemon_Fanfare::
	playfanfare MUS_OBTAIN_ITEM
	bufferspeciesname STR_VAR_1, VAR_TEMP_TRANSFERRED_SPECIES
	message Randomizer_Message_Text
	waitmessage
	waitfanfare
	return

Randomizer_Message_Text:
	.string "{PLAYER} received {STR_VAR_1}!$"