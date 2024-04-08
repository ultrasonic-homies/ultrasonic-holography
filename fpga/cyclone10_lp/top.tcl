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
# Generated on: Mon Apr 08 12:14:46 2024

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
	set_global_assignment -name PROJECT_CREATION_TIME_DATE "20:50:27  ÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂ¸ÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂ½ÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂÃÂ 05, 2021"
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
	set_global_assignment -name SDC_FILE cyclone10_lp.sdc
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/top.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/receiver.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/pwm.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/phase_parser.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/sync_receiver.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/sync_sender.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../shared/modulation.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../proto245/src/proto245a.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../proto245/src/fifo_async.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../proto245/src/fifo_sync.sv
	set_global_assignment -name SYSTEMVERILOG_FILE ../proto245/src/dpram.sv
	set_global_assignment -name QIP_FILE ../ip/PLL_24M576_TO_10M24_CYCLONE10LP.qip
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_clk
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_rxfn
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_txen
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_rdn
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_wrn
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_siwu
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_oen
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to sync_in
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to sync_out
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[4]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[5]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[7]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[8]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[9]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[10]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[11]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[12]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[13]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[14]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[15]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[16]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[17]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[18]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[19]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[20]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[21]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[22]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[23]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[24]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[25]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[26]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[27]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[28]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[29]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[30]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[31]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[32]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[33]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[34]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[35]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[36]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[37]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[38]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[39]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[40]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[41]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[42]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[43]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[44]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[45]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[46]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[47]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[48]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[49]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[50]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[51]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[52]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[53]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[54]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[55]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[56]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[57]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[58]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[59]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[60]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[61]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[62]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[63]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[64]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[65]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[66]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[67]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[68]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[69]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[70]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[71]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[72]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[73]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[74]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[75]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[76]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[77]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[78]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[79]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[80]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[81]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[82]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[83]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[84]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[85]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[86]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[87]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[88]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[89]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[90]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[91]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[92]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[93]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[94]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[95]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[96]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[97]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[98]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[99]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[100]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[101]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[102]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[103]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[104]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[105]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[106]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[107]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[108]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[109]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[110]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[111]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[112]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[113]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[114]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[115]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[116]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[117]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[118]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[119]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[120]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[121]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[122]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[123]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[124]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[125]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[126]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[127]
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
