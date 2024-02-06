import os
import json

PINOUT_FILE_PATH = "pinout.json"

FTCHIP_ORDER = [
    "ft_data[0]",
    "ft_data[1]",
    "ft_data[2]",
    "ft_data[3]",
    "ft_data[4]",
    "ft_data[5]",
    "ft_data[6]",
    "ft_data[7]",
    "ft_txen",
    "ft_rxfn",
    "ft_clk",
    "ft_oen",
    "ft_siwu",
]

SYS_ORDER = [
    "sync_in",
    "sync_out",
    "ext_rst",
]

def generate_lines(pin_names, net_names):
    lines = []
    if len(net_names) != len(pin_names):
        print("Error: specified and expected number of pins do not match")
        return lines
    for i, pin in enumerate(pin_names):
        lines.append(f"set_location_assignment PIN_{pin} -to {net_names[i]}\n")
        lines.append(f"set_instance_assignment -name IO_STANDARD \"3.3-V LVTTL\" -to {net_names[i]}\n")
    return lines

def main():
    pinout_data = {}
    with open(os.path.join(os.path.dirname(os.path.realpath(__file__)),f"{PINOUT_FILE_PATH}")) as pinout_file:
        pinout_data = json.load(pinout_file)
    generated_file_name = os.path.join(os.path.dirname(os.path.realpath(__file__)),f"{pinout_data['name']}.qsf")
    lines = [
        "#============================================================\n",
        f"# Generated Pin Assignments for {pinout_data['name']}\n",
        "#============================================================\n\n\n",
    ]
    lines += generate_lines(pinout_data['transducer_pins'], [f"trans[{i}]" for i in range(len(pinout_data['transducer_pins']))])
    lines += generate_lines(pinout_data['ftdi_pins'], FTCHIP_ORDER)
    lines += generate_lines(pinout_data['sys_pins'], SYS_ORDER)
    with open(generated_file_name, 'w') as tcl_file:
        tcl_file.writelines(lines)
    print(f"Pin assignments generated in {generated_file_name}")

if __name__ == "__main__":
    main()
