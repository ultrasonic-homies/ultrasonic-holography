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
# File: de1_soc_top.tcl
# Generated on: Fri Dec 15 15:28:11 2023

# Load Quartus Prime Tcl Project package
package require ::quartus::project

set need_to_close_project 0
set make_assignments 1

# Check that the right project is open
if {[is_project_open]} {
	if {[string compare $quartus(project) "de1_soc_top"]} {
		puts "Project de1_soc_top is not open"
		set make_assignments 0
	}
} else {
	# Only open if not already open
	if {[project_exists de1_soc_top]} {
		project_open -revision de1_soc_top de1_soc_top
	} else {
		project_new -revision de1_soc_top de1_soc_top
	}
	set need_to_close_project 1
}

# Make assignments
if {$make_assignments} {
	set_global_assignment -name FAMILY "Cyclone V"
	set_global_assignment -name DEVICE 5CSEMA5F31C6
	set_global_assignment -name ORIGINAL_QUARTUS_VERSION 19.1.0
	set_global_assignment -name PROJECT_CREATION_TIME_DATE "20:50:27  ÃÂ¸ÃÂÃÂ½ÃÂ 05, 2021"
	set_global_assignment -name LAST_QUARTUS_VERSION "19.1.0 Lite Edition"
	set_global_assignment -name DEVICE_FILTER_PACKAGE FBGA
	set_global_assignment -name PROJECT_OUTPUT_DIRECTORY output_files
	set_global_assignment -name MIN_CORE_JUNCTION_TEMP 0
	set_global_assignment -name MAX_CORE_JUNCTION_TEMP 85
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
	set_global_assignment -name POWER_PRESET_COOLING_SOLUTION "23 MM HEAT SINK WITH 200 LFPM AIRFLOW"
	set_global_assignment -name POWER_BOARD_THERMAL_MODEL "NONE (CONSERVATIVE)"
	set_global_assignment -name VERILOG_INPUT_VERSION SYSTEMVERILOG_2005
	set_global_assignment -name VERILOG_SHOW_LMF_MAPPING_MESSAGES OFF
	set_global_assignment -name OPTIMIZATION_MODE BALANCED
	set_global_assignment -name BOARD "DE1-SoC Board"
	set_global_assignment -name PARTITION_NETLIST_TYPE SOURCE -section_id Top
	set_global_assignment -name PARTITION_FITTER_PRESERVATION_LEVEL PLACEMENT_AND_ROUTING -section_id Top
	set_global_assignment -name PARTITION_COLOR 16764057 -section_id Top
	set_global_assignment -name SYSTEMVERILOG_FILE de1_soc_top.sv
	set_global_assignment -name SDC_FILE de1_soc_top.sdc
	set_global_assignment -name SYSTEMVERILOG_FILE top.sv
	set_global_assignment -name SYSTEMVERILOG_FILE receiver.sv
	set_global_assignment -name SYSTEMVERILOG_FILE pwm.sv
	set_global_assignment -name SYSTEMVERILOG_FILE proto245/src/proto245a.sv
	set_global_assignment -name SYSTEMVERILOG_FILE proto245/src/fifo_sync.sv
	set_global_assignment -name SYSTEMVERILOG_FILE proto245/src/fifo_async.sv
	set_global_assignment -name SYSTEMVERILOG_FILE proto245/src/dpram.sv
	set_global_assignment -name QIP_FILE ip/pll50.qip
	set_location_assignment PIN_AF14 -to CLOCK_50
	set_location_assignment PIN_AA14 -to KEY[0]
	set_location_assignment PIN_AA15 -to KEY[1]
	set_location_assignment PIN_W15 -to KEY[2]
	set_location_assignment PIN_Y16 -to KEY[3]
	set_location_assignment PIN_V16 -to LEDR[0]
	set_location_assignment PIN_W16 -to LEDR[1]
	set_location_assignment PIN_V17 -to LEDR[2]
	set_location_assignment PIN_V18 -to LEDR[3]
	set_location_assignment PIN_W17 -to LEDR[4]
	set_location_assignment PIN_W19 -to LEDR[5]
	set_location_assignment PIN_Y19 -to LEDR[6]
	set_location_assignment PIN_W20 -to LEDR[7]
	set_location_assignment PIN_W21 -to LEDR[8]
	set_location_assignment PIN_Y21 -to LEDR[9]
	set_location_assignment PIN_AB12 -to SW[0]
	set_location_assignment PIN_AC12 -to SW[1]
	set_location_assignment PIN_AF9 -to SW[2]
	set_location_assignment PIN_AF10 -to SW[3]
	set_location_assignment PIN_AD11 -to SW[4]
	set_location_assignment PIN_AD12 -to SW[5]
	set_location_assignment PIN_AE11 -to SW[6]
	set_location_assignment PIN_AC9 -to SW[7]
	set_location_assignment PIN_AD10 -to SW[8]
	set_location_assignment PIN_AE12 -to SW[9]
	set_location_assignment PIN_AD17 -to ft_clk
	set_location_assignment PIN_AJ17 -to ft_data[0]
	set_location_assignment PIN_AH18 -to ft_data[1]
	set_location_assignment PIN_AG16 -to ft_data[2]
	set_location_assignment PIN_AF16 -to ft_data[3]
	set_location_assignment PIN_AA18 -to ft_data[4]
	set_location_assignment PIN_AE17 -to ft_data[5]
	set_location_assignment PIN_AH19 -to ft_data[6]
	set_location_assignment PIN_AH20 -to ft_data[7]
	set_location_assignment PIN_AD19 -to ft_rxfn
	set_location_assignment PIN_AE18 -to ft_txen
	set_location_assignment PIN_AF20 -to ft_rdn
	set_location_assignment PIN_AF19 -to ft_wrn
	set_location_assignment PIN_AF18 -to ft_siwu
	set_location_assignment PIN_AG18 -to ft_oen
	set_location_assignment PIN_AB17 -to sync_in
	set_location_assignment PIN_AB21 -to sync_out
	set_location_assignment PIN_AH22 -to trans[0]
	set_location_assignment PIN_AF24 -to trans[1]
	set_location_assignment PIN_AE22 -to trans[2]
	set_location_assignment PIN_AA20 -to trans[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to CLOCK_50
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to KEY[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to KEY[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to KEY[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to KEY[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to LEDR[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to LEDR[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to LEDR[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to LEDR[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to LEDR[4]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to LEDR[5]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to LEDR[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to LEDR[7]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to LEDR[8]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to LEDR[9]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SW[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SW[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SW[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SW[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SW[4]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SW[5]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SW[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SW[7]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SW[8]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to SW[9]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_clk
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_data[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_data[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_data[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_data[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_data[4]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_data[5]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_data[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to ft_data[7]
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
	set_location_assignment PIN_AE26 -to HEX0[0]
	set_location_assignment PIN_AE27 -to HEX0[1]
	set_location_assignment PIN_AE28 -to HEX0[2]
	set_location_assignment PIN_AG27 -to HEX0[3]
	set_location_assignment PIN_AF28 -to HEX0[4]
	set_location_assignment PIN_AG28 -to HEX0[5]
	set_location_assignment PIN_AH28 -to HEX0[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX0[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX0[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX0[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX0[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX0[4]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX0[5]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX0[6]
	set_location_assignment PIN_AJ29 -to HEX1[0]
	set_location_assignment PIN_AH29 -to HEX1[1]
	set_location_assignment PIN_AH30 -to HEX1[2]
	set_location_assignment PIN_AG30 -to HEX1[3]
	set_location_assignment PIN_AF29 -to HEX1[4]
	set_location_assignment PIN_AF30 -to HEX1[5]
	set_location_assignment PIN_AD27 -to HEX1[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX1[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX1[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX1[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX1[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX1[4]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX1[5]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX1[6]
	set_location_assignment PIN_AB23 -to HEX2[0]
	set_location_assignment PIN_AE29 -to HEX2[1]
	set_location_assignment PIN_AD29 -to HEX2[2]
	set_location_assignment PIN_AC28 -to HEX2[3]
	set_location_assignment PIN_AD30 -to HEX2[4]
	set_location_assignment PIN_AC29 -to HEX2[5]
	set_location_assignment PIN_AC30 -to HEX2[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX2[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX2[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX2[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX2[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX2[4]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX2[5]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX2[6]
	set_location_assignment PIN_AD26 -to HEX3[0]
	set_location_assignment PIN_AC27 -to HEX3[1]
	set_location_assignment PIN_AD25 -to HEX3[2]
	set_location_assignment PIN_AC25 -to HEX3[3]
	set_location_assignment PIN_AB28 -to HEX3[4]
	set_location_assignment PIN_AB25 -to HEX3[5]
	set_location_assignment PIN_AB22 -to HEX3[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX3[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX3[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX3[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX3[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX3[4]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX3[5]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX3[6]
	set_location_assignment PIN_AA24 -to HEX4[0]
	set_location_assignment PIN_Y23 -to HEX4[1]
	set_location_assignment PIN_Y24 -to HEX4[2]
	set_location_assignment PIN_W22 -to HEX4[3]
	set_location_assignment PIN_W24 -to HEX4[4]
	set_location_assignment PIN_V23 -to HEX4[5]
	set_location_assignment PIN_W25 -to HEX4[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX4[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX4[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX4[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX4[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX4[4]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX4[5]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX4[6]
	set_location_assignment PIN_V25 -to HEX5[0]
	set_location_assignment PIN_AA28 -to HEX5[1]
	set_location_assignment PIN_Y27 -to HEX5[2]
	set_location_assignment PIN_AB27 -to HEX5[3]
	set_location_assignment PIN_AB26 -to HEX5[4]
	set_location_assignment PIN_AA26 -to HEX5[5]
	set_location_assignment PIN_AA25 -to HEX5[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX5[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX5[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX5[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX5[3]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX5[4]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX5[5]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to HEX5[6]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to sync_in
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to sync_out
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[0]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[1]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[2]
	set_instance_assignment -name IO_STANDARD "3.3-V LVTTL" -to trans[3]
	set_instance_assignment -name PARTITION_HIERARCHY root_partition -to | -section_id Top

	# Commit assignments
	export_assignments

	# Close project
	if {$need_to_close_project} {
		project_close
	}
}
