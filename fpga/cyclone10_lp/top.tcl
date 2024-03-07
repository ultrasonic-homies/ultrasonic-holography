# Copyright (C) 2019  Intel Corporation. All rights reserved.
# Your use of Intel Corporation's design tools, logic functions 
# and other software and tools, and any partner logic 
# functions, and any output files from any of the foregoing 
# (including device programming or simulation files), and any 
# associated documentation or information are expressly subject 
# to the terms and conditions of the Intel Program License 
# Subscription Agreement, the Intel Quartus Prime License Agreement,
# the Intel FPGA IP License Agreement, or other applicable license
# agreement, including, without limitation, that your use is for
# the sole purpose of programming logic devices manufactured by
# Intel and sold by Intel or its authorized distributors.  Please
# refer to the applicable agreement for further details, at
# https://fpgasoftware.intel.com/eula.

# Quartus Prime: Generate Tcl File for Project
# File: top.tcl
# Generated on: Thu Mar 07 13:51:59 2024

# Load Quartus Prime Tcl Project package
package require ::quartus::project

set need_to_close_project 0
set make_assignments 1

# Check that the right project is open
if {[is_project_open]} {
	if {[string compare $quartus(project) "top"]} {
		puts "Project top is not open"
		set make_assignments 0
	}
} else {
	# Only open if not already open
	if {[project_exists top]} {
		project_open -revision top top
	} else {
		project_new -revision top top
	}
	set need_to_close_project 1
}

# Make assignments
if {$make_assignments} {
	set_global_assignment -name FAMILY "Cyclone 10 LP"
	set_global_assignment -name DEVICE 10CL010YU256A7G
	set_global_assignment -name ORIGINAL_QUARTUS_VERSION 19.1.0
	set_global_assignment -name PROJECT_CREATION_TIME_DATE "20:50:27  ÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂ¸ÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂ½ÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂ 05, 2021"
	set_global_assignment -name LAST_QUARTUS_VERSION "19.1.0 Lite Edition"
	set_global_assignment -name PROJECT_OUTPUT_DIRECTORY output_files
	set_global_assignment -name MIN_CORE_JUNCTION_TEMP "-40"
	set_global_assignment -name MAX_CORE_JUNCTION_TEMP 125
	set_global_assignment -name ERROR_CHECK_FREQUENCY_DIVISOR 256
	set_global_assignment -name EDA_SIMULATION_TOOL "ModelSim-Altera (Verilog)"
	set_global_assignment -name EDA_TIME_SCALE "1 ps" -section_id eda_simulation
	set_global_assignment -name EDA_OUTPUT_DATA_FORMAT "VERILOG HDL" -section_id eda_simulation
	set_global_assignment -name EDA_DESIGN_ENTRY_SYNTHESIS_TOOL "Precision Synthesis"
	set_global_assignment -name EDA_LMF_FILE mentor.lmf -section_id eda_design_synthesis
	set_global_assignment -name EDA_INPUT_DATA_FORMAT VQM -section_id eda_design_synthesis
	set_global_assignment -name EDA_GENERATE_FUNCTIONAL_NETLIST OFF -section_id eda_board_design_timing
	set_global_assignment -name EDA_GENERATE_FUNCTIONAL_NETLIST OFF -section_id eda_board_design_symbol
	set_global_assignment -name EDA_GENERATE_FUNCTIONAL_NETLIST OFF -section_id eda_board_design_signal_integrity
	set_global_assignment -name EDA_GENERATE_FUNCTIONAL_NETLIST OFF -section_id eda_board_design_boundary_scan
	set_global_assignment -name POWER_PRESET_COOLING_SOLUTION "NO HEAT SINK WITH 100 LFPM AIRFLOW"
	set_global_assignment -name POWER_BOARD_THERMAL_MODEL "NONE (CONSERVATIVE)"
	set_global_assignment -name VERILOG_INPUT_VERSION SYSTEMVERILOG_2005
	set_global_assignment -name VERILOG_SHOW_LMF_MAPPING_MESSAGES OFF
	set_global_assignment -name OPTIMIZATION_MODE BALANCED
	set_global_assignment -name PARTITION_NETLIST_TYPE SOURCE -section_id Top
	set_global_assignment -name PARTITION_FITTER_PRESERVATION_LEVEL PLACEMENT_AND_ROUTING -section_id Top
	set_global_assignment -name PARTITION_COLOR 16764057 -section_id Top
	set_global_assignment -name SDC_FILE cyclone10_lp.sdc
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/top.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/receiver.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/pwm.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/phase_parser.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/sync_receiver.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/sync_sender.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../proto245/src/proto245a.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../proto245/src/fifo_async.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../proto245/src/fifo_sync.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../proto245/src/dpram.sv
	set_global_assignment -name QIP_FILE ../ip/PLL_24M576_TO_10M24_CYCLONE10LP.qip
	set_global_assignment -name ENABLE_OCT_DONE OFF
	set_global_assignment -name USE_CONFIGURATION_DEVICE ON
	set_global_assignment -name CYCLONEIII_CONFIGURATION_DEVICE EPCQ16A
	set_global_assignment -name CRC_ERROR_OPEN_DRAIN OFF
	set_global_assignment -name FORCE_CONFIGURATION_VCCIO ON
	set_global_assignment -name CONFIGURATION_VCCIO_LEVEL 3.3V
	set_global_assignment -name CYCLONEII_RESERVE_NCEO_AFTER_CONFIGURATION "USE AS REGULAR IO"
	set_global_assignment -name OUTPUT_IO_TIMING_NEAR_END_VMEAS "HALF VCCIO" -rise
	set_global_assignment -name OUTPUT_IO_TIMING_NEAR_END_VMEAS "HALF VCCIO" -fall
	set_global_assignment -name OUTPUT_IO_TIMING_FAR_END_VMEAS "HALF SIGNAL SWING" -rise
	set_global_assignment -name OUTPUT_IO_TIMING_FAR_END_VMEAS "HALF SIGNAL SWING" -fall
	set_global_assignment -name ENABLE_CONFIGURATION_PINS OFF
	set_global_assignment -name ENABLE_BOOT_SEL_PIN OFF
	set_global_assignment -name STRATIX_DEVICE_IO_STANDARD "3.3-V LVTTL"
	set_global_assignment -name POWER_USE_INPUT_FILES OFF
	set_global_assignment -name POWER_OUTPUT_SAF_NAME "C:\\Users\\T460s_User\\Documents\\School\\ENPH479\\ultrasonic-holography\\fpga\\cyclone10_lp\\output_files\\signal_activities.saf"
	set_global_assignment -name FLOW_ENABLE_POWER_ANALYZER ON
	set_global_assignment -name POWER_DEFAULT_INPUT_IO_TOGGLE_RATE 12.5%
	set_global_assignment -name POWER_REPORT_SIGNAL_ACTIVITY ON
	set_global_assignment -name POWER_REPORT_POWER_DISSIPATION ON
	set_location_assignment PIN_K10 -to ft_clk
	set_location_assignment PIN_A9 -to ft_data[0]
	set_location_assignment PIN_B9 -to ft_data[1]
	set_location_assignment PIN_A10 -to ft_data[2]
	set_location_assignment PIN_B10 -to ft_data[3]
	set_location_assignment PIN_A11 -to ft_data[4]
	set_location_assignment PIN_B11 -to ft_data[5]
	set_location_assignment PIN_A12 -to ft_data[6]
	set_location_assignment PIN_B12 -to ft_data[7]
	set_location_assignment PIN_A13 -to ft_rxfn
	set_location_assignment PIN_B13 -to ft_txen
	set_location_assignment PIN_A15 -to ft_siwu
	set_location_assignment PIN_B16 -to ft_oen
	set_location_assignment PIN_A2 -to sync_in
	set_location_assignment PIN_K6 -to sync_out
	set_location_assignment PIN_C3 -to trans[0]
	set_location_assignment PIN_B5 -to trans[1]
	set_location_assignment PIN_D6 -to trans[2]
	set_location_assignment PIN_E7 -to trans[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_clk
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_rxfn
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_txen
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_rdn
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_wrn
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_siwu
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_oen
	set_instance_assignment -name FAST_OUTPUT_REGISTER ON -to ft_rdn
	set_instance_assignment -name FAST_INPUT_REGISTER ON -to ft_rxfn
	set_instance_assignment -name FAST_INPUT_REGISTER ON -to ft_txen
	set_instance_assignment -name FAST_OUTPUT_REGISTER ON -to ft_wrn
	set_instance_assignment -name FAST_OUTPUT_REGISTER ON -to ft_data
	set_instance_assignment -name FAST_INPUT_REGISTER ON -to ft_data
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to sync_in
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to sync_out
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[3]
	set_location_assignment PIN_E6 -to trans[4]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[4]
	set_location_assignment PIN_A6 -to trans[5]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[5]
	set_location_assignment PIN_D8 -to trans[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[6]
	set_location_assignment PIN_B8 -to trans[7]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[7]
	set_location_assignment PIN_D12 -to trans[8]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[8]
	set_location_assignment PIN_D14 -to trans[9]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[9]
	set_location_assignment PIN_F10 -to trans[10]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[10]
	set_location_assignment PIN_B1 -to trans[11]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[11]
	set_location_assignment PIN_C11 -to trans[12]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[12]
	set_location_assignment PIN_E9 -to trans[13]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[13]
	set_location_assignment PIN_D9 -to trans[14]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[14]
	set_location_assignment PIN_E8 -to trans[15]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[15]
	set_location_assignment PIN_F9 -to trans[16]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[16]
	set_location_assignment PIN_C16 -to trans[17]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[17]
	set_location_assignment PIN_D16 -to trans[18]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[18]
	set_location_assignment PIN_F11 -to trans[19]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[19]
	set_location_assignment PIN_J11 -to trans[20]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[20]
	set_location_assignment PIN_F13 -to trans[21]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[21]
	set_location_assignment PIN_D1 -to trans[22]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[22]
	set_location_assignment PIN_G15 -to trans[23]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[23]
	set_location_assignment PIN_J12 -to trans[24]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[24]
	set_location_assignment PIN_J13 -to trans[25]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[25]
	set_location_assignment PIN_N14 -to trans[26]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[26]
	set_location_assignment PIN_P16 -to trans[27]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[27]
	set_location_assignment PIN_K11 -to trans[28]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[28]
	set_location_assignment PIN_N16 -to trans[29]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[29]
	set_location_assignment PIN_L12 -to trans[30]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[30]
	set_location_assignment PIN_L13 -to trans[31]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[31]
	set_location_assignment PIN_L16 -to trans[32]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[32]
	set_location_assignment PIN_D3 -to trans[33]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[33]
	set_location_assignment PIN_K16 -to trans[34]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[34]
	set_location_assignment PIN_R16 -to trans[35]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[35]
	set_location_assignment PIN_R14 -to trans[36]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[36]
	set_location_assignment PIN_L11 -to trans[37]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[37]
	set_location_assignment PIN_T13 -to trans[38]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[38]
	set_location_assignment PIN_N12 -to trans[39]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[39]
	set_location_assignment PIN_P11 -to trans[40]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[40]
	set_location_assignment PIN_T9 -to trans[41]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[41]
	set_location_assignment PIN_R10 -to trans[42]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[42]
	set_location_assignment PIN_T4 -to trans[43]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[43]
	set_location_assignment PIN_G1 -to trans[44]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[44]
	set_location_assignment PIN_T3 -to trans[45]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[45]
	set_location_assignment PIN_M7 -to trans[46]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[46]
	set_location_assignment PIN_T5 -to trans[47]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[47]
	set_location_assignment PIN_T6 -to trans[48]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[48]
	set_location_assignment PIN_P6 -to trans[49]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[49]
	set_location_assignment PIN_R7 -to trans[50]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[50]
	set_location_assignment PIN_R8 -to trans[51]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[51]
	set_location_assignment PIN_T2 -to trans[52]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[52]
	set_location_assignment PIN_P3 -to trans[53]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[53]
	set_location_assignment PIN_P1 -to trans[54]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[54]
	set_location_assignment PIN_F3 -to trans[55]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[55]
	set_location_assignment PIN_N1 -to trans[56]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[56]
	set_location_assignment PIN_N3 -to trans[57]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[57]
	set_location_assignment PIN_L3 -to trans[58]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[58]
	set_location_assignment PIN_K1 -to trans[59]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[59]
	set_location_assignment PIN_K5 -to trans[60]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[60]
	set_location_assignment PIN_G2 -to trans[61]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[61]
	set_location_assignment PIN_J1 -to trans[62]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[62]
	set_location_assignment PIN_B4 -to trans[63]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[63]
	set_location_assignment PIN_C2 -to trans[64]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[64]
	set_location_assignment PIN_A5 -to trans[65]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[65]
	set_location_assignment PIN_C6 -to trans[66]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[66]
	set_location_assignment PIN_F7 -to trans[67]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[67]
	set_location_assignment PIN_B6 -to trans[68]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[68]
	set_location_assignment PIN_B7 -to trans[69]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[69]
	set_location_assignment PIN_A8 -to trans[70]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[70]
	set_location_assignment PIN_A7 -to trans[71]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[71]
	set_location_assignment PIN_C14 -to trans[72]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[72]
	set_location_assignment PIN_C15 -to trans[73]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[73]
	set_location_assignment PIN_E10 -to trans[74]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[74]
	set_location_assignment PIN_A3 -to trans[75]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[75]
	set_location_assignment PIN_D11 -to trans[76]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[76]
	set_location_assignment PIN_C9 -to trans[77]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[77]
	set_location_assignment PIN_E11 -to trans[78]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[78]
	set_location_assignment PIN_C8 -to trans[79]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[79]
	set_location_assignment PIN_F8 -to trans[80]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[80]
	set_location_assignment PIN_D15 -to trans[81]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[81]
	set_location_assignment PIN_F15 -to trans[82]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[82]
	set_location_assignment PIN_G11 -to trans[83]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[83]
	set_location_assignment PIN_F16 -to trans[84]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[84]
	set_location_assignment PIN_F14 -to trans[85]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[85]
	set_location_assignment PIN_E5 -to trans[86]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[86]
	set_location_assignment PIN_G16 -to trans[87]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[87]
	set_location_assignment PIN_J14 -to trans[88]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[88]
	set_location_assignment PIN_J16 -to trans[89]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[89]
	set_location_assignment PIN_N13 -to trans[90]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[90]
	set_location_assignment PIN_P15 -to trans[91]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[91]
	set_location_assignment PIN_K12 -to trans[92]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[92]
	set_location_assignment PIN_N15 -to trans[93]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[93]
	set_location_assignment PIN_L14 -to trans[94]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[94]
	set_location_assignment PIN_M12 -to trans[95]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[95]
	set_location_assignment PIN_K15 -to trans[96]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[96]
	set_location_assignment PIN_D4 -to trans[97]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[97]
	set_location_assignment PIN_J15 -to trans[98]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[98]
	set_location_assignment PIN_T15 -to trans[99]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[99]
	set_location_assignment PIN_T14 -to trans[100]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[100]
	set_location_assignment PIN_R13 -to trans[101]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[101]
	set_location_assignment PIN_R12 -to trans[102]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[102]
	set_location_assignment PIN_N11 -to trans[103]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[103]
	set_location_assignment PIN_T12 -to trans[104]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[104]
	set_location_assignment PIN_R9 -to trans[105]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[105]
	set_location_assignment PIN_R11 -to trans[106]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[106]
	set_location_assignment PIN_R4 -to trans[107]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[107]
	set_location_assignment PIN_F5 -to trans[108]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[108]
	set_location_assignment PIN_R3 -to trans[109]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[109]
	set_location_assignment PIN_R6 -to trans[110]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[110]
	set_location_assignment PIN_R5 -to trans[111]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[111]
	set_location_assignment PIN_M6 -to trans[112]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[112]
	set_location_assignment PIN_N6 -to trans[113]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[113]
	set_location_assignment PIN_T7 -to trans[114]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[114]
	set_location_assignment PIN_T8 -to trans[115]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[115]
	set_location_assignment PIN_R1 -to trans[116]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[116]
	set_location_assignment PIN_P2 -to trans[117]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[117]
	set_location_assignment PIN_N2 -to trans[118]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[118]
	set_location_assignment PIN_F6 -to trans[119]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[119]
	set_location_assignment PIN_L6 -to trans[120]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[120]
	set_location_assignment PIN_L4 -to trans[121]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[121]
	set_location_assignment PIN_L2 -to trans[122]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[122]
	set_location_assignment PIN_K2 -to trans[123]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[123]
	set_location_assignment PIN_L1 -to trans[124]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[124]
	set_location_assignment PIN_J6 -to trans[125]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[125]
	set_location_assignment PIN_J2 -to trans[126]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[126]
	set_location_assignment PIN_A4 -to trans[127]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[127]
	set_location_assignment PIN_B3 -to ext_rst
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ext_rst
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_data
	set_instance_assignment -name PARTITION_HIERARCHY root_partition -to | -section_id Top

	# Commit assignments
	export_assignments

	# Close project
	if {$need_to_close_project} {
		project_close
	}
}
