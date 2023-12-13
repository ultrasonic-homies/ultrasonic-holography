set_global_assignment -entity "pll50" -library "lib_pll50" -name IP_TOOL_NAME "altera_pll"
set_global_assignment -entity "pll50" -library "lib_pll50" -name IP_TOOL_VERSION "19.1"
set_global_assignment -entity "pll50" -library "lib_pll50" -name IP_TOOL_ENV "mwpim"
set_global_assignment -library "lib_pll50" -name SPD_FILE [file join $::quartus(sip_path) "pll50.spd"]

set_global_assignment -library "lib_pll50" -name MISC_FILE [file join $::quartus(sip_path) "pll50_sim/pll50.vo"]
