DRIVERS_PER_FPGA = 64

# key: Altium net name
# value: SystemVerilog net name

FPGA_PINOUT = {
    "FIFO0": "ft_data[0]",
    "FIFO1": "ft_data[1]",
    "FIFO2": "ft_data[2]",
    "FIFO3": "ft_data[3]",
    "FIFO4": "ft_data[4]",
    "FIFO5": "ft_data[5]",
    "FIFO6": "ft_data[6]",
    "FIFO7": "ft_data[7]",
    "T\\X\\E\\": "ft_txen",
    "R\\C\\X\\F\\": "ft_rxfn",
    "OEN": "ft_oen",
    "SIWU": "ft_siwu",
    "WRN": "ft_wrn",
    "RDN": "ft_rdn",
    "RST": "ext_rst",
    **{f"IO_A_{i+1}": f"trans[{2*i}]" for i in range(DRIVERS_PER_FPGA)},
    **{f"IO_B_{i+1}": f"trans[{2*i+1}]" for i in range(DRIVERS_PER_FPGA)}
}

FPGA_PRIMARY_PINOUT = {
    "CLK_OUT": "sync_out",
    **FPGA_PINOUT
}

FPGA_SECONDARY_PINOUT = {
    "CLK_IN": "sync_in",
    **FPGA_PINOUT
}