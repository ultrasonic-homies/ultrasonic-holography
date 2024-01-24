import os

GPIO_PINOUT =   "cyclone10_10cl006_u256_first128"
FTCHIP_PINOUT = "cyclone10_10cl006_u256_ftchip"
SYS_PINOUT =    "cyclone10_10cl006_u256_sys"

PIN_ASSIGNMENTS = {
    "cyclone10_10cl010_u256_all": [
        "D4","E5","F5","B1","C2","C1","F3","D2","D1","G2",
        "G1","H1","H2","J2","J1","J6","K6","L6","K2","K1",
        "L2","L1","L3","N2","N1","K5","L4","R1","P2","P1",
        "N3","P3","R3","T3","T2","R4","T4","N5","N6","M6",
        "P6","M7","K8","R5","T5","R6","T6","L7","R7","T7",
        "L8","M8","N8","P8","R8","T8","R9","T9","K9","L9",
        "M9","N9","R10","T10","R11","T11","R12","T12","K10","L10",
        "P9","P11","R13","T13","M10","N11","T14","T15","R14","P14",
        "L11","M11","N12","N13","M12","L12","K12","N14","P15","P16",
        "R16","K11","N16","N15","L14","L13","L16","L15","J11","K16",
        "K15","J16","J15","J14","J12","J13","G16","G15","F13","F16",
        "F15","B16","F14","D16","D15","G11","C16","C15","C14","D14",
        "D11","D12","A13","B13","A14","B14","E11","E10","A12","B12",
        "A11","B11","C11","F10","F9","F11","A15","A10","B10","C9",
        "D9","E9","A9","B9","A8","B8","C8","D8","E8","F8",
        "A7","B7","F6","F7","C6","A6","B6","E7","E6","A5",
        "A2","B5","A4","B4","D6","A3","B3","C3","D3",
    ],
    "cyclone10_10cl010_u256_first128": [
        "D4","E5","F5","B1","C2","C1","F3","D2","D1","G2",
        "G1","H1","H2","J2","J1","J6","K6","L6","K2","K1",
        "L2","L1","L3","N2","N1","K5","L4","R1","P2","P1",
        "N3","P3","R3","T3","T2","R4","T4","N5","N6","M6",
        "P6","M7","K8","R5","T5","R6","T6","L7","R7","T7",
        "L8","M8","N8","P8","R8","T8","R9","T9","K9","L9",
        "M9","N9","R10","T10","R11","T11","R12","T12","K10","L10",
        "P9","P11","R13","T13","M10","N11","T14","T15","R14","P14",
        "L11","M11","N12","N13","M12","L12","K12","N14","P15","P16",
        "R16","K11","N16","N15","L14","L13","L16","L15","J11","K16",
        "K15","J16","J15","J14","J12","J13","G16","G15","F13","F16",
        "F15","B16","F14","D16","D15","G11","C16","C15","C14","D14",
        "D11","D12","A13","B13","A14","B14","E11","E10",
    ],
    "cyclone10_10cl010_u256_ftchip": [
        "A12","B12","A11","B11","C11","F10","F9","F11","A15","A10","B10","C9","D9"
    ],
    "cyclone10_10cl010_u256_sys": [
        "E9","A9","B9"
    ],
    "cyclone10_10cl006_u256_all": [
        "D4","E5","F5","B1","C2","F3","D1","G2","G1","J2",
        "J1","J6","K6","L6","K2","K1","L2","L1","L3","N2",
        "N1","K5","L4","R1","P2","P1","N3","P3","R3","T3",
        "T2","R4","T4","N5","N6","M6","P6","M7","K8","R5",
        "T5","R6","T6","L7","R7","T7","L8","M8","N8","P8",
        "R8","T8","R9","T9","K9","L9","M9","N9","R10","T10",
        "R11","T11","R12","T12","K10","L10","P9","P11","R13","T13",
        "M10","N11","T14","T15","R14","P14","L11","M11","N12","N13",
        "M12","L12","K12","N14","P15","P16","R16","K11","N16","N15",
        "L14","L13","L16","L15","J11","K16","K15","J14","J12","J13",
        "F13","B16","F14","D16","D15","G11","C16","C15","C14","D14",
        "D11","D12","A13","B13","A14","B14","E11","E10","A12","B12",
        "A11","B11","C11","F10","F9","F11","A15","A10","B10","C9",
        "D9","E9","A9","B9","A8","B8","C8","D8","A7","F6",
        "F7","C6","A6","B6","A2","B5","A4","B4","D6","A3",
        "B3","C3","D3",
    ],
    "cyclone10_10cl006_u256_first128": [
        "D4","E5","F5","B1","C2","F3","D1","G2","G1","J2",
        "J1","J6","K6","L6","K2","K1","L2","L1","L3","N2",
        "N1","K5","L4","R1","P2","P1","N3","P3","R3","T3",
        "T2","R4","T4","N5","N6","M6","P6","M7","K8","R5",
        "T5","R6","T6","L7","R7","T7","L8","M8","N8","P8",
        "R8","T8","R9","T9","K9","L9","M9","N9","R10","T10",
        "R11","T11","R12","T12","K10","L10","P9","P11","R13","T13",
        "M10","N11","T14","T15","R14","P14","L11","M11","N12","N13",
        "M12","L12","K12","N14","P15","P16","R16","K11","N16","N15",
        "L14","L13","L16","L15","J11","K16","K15","J14","J12","J13",
        "F13","B16","F14","D16","D15","G11","C16","C15","C14","D14",
        "D11","D12","A13","B13","A14","B14","E11","E10","A12","B12",
        "A11","B11","C11","F10","F9","F11","A15","A10",
    ],
    "cyclone10_10cl006_u256_ftchip": [
        "B10","C9","D9","E9","A9","B9","A8","B8","C8","D8","A7","F6","F7",
    ],
    "cyclone10_10cl006_u256_sys": [
        "C6","A6","B6",
    ]
}
GENERATED_FILE_NAME = os.path.join(os.path.dirname(os.path.realpath(__file__)),f"{GPIO_PINOUT}.qsf")

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

def generate_lines(pinout, names):
    lines = []
    pins = PIN_ASSIGNMENTS[pinout]
    if len(names) != len(pins):
        print("specified and expected number of pins do not match")
    for i, pin in enumerate(pins):
        lines.append(f"set_location_assignment PIN_{pin} -to {names[i]}\n")
        lines.append(f"set_instance_assignment -name IO_STANDARD \"3.3-V LVTTL\" -to {names[i]}\n")
        if i + 1 >= len(names):
            break
    return lines

def main():
    lines = [
        "#============================================================\n",
        f"# Generated Pin Assignments for {GPIO_PINOUT}\n",
        "#============================================================\n\n\n",
    ]
    lines += generate_lines(GPIO_PINOUT, [f"trans[{i}]" for i in range(len(PIN_ASSIGNMENTS[GPIO_PINOUT]))])
    lines += generate_lines(FTCHIP_PINOUT, FTCHIP_ORDER)
    lines += generate_lines(SYS_PINOUT, SYS_ORDER)
    with open(GENERATED_FILE_NAME, 'w') as tcl_file:
        tcl_file.writelines(lines)
    print(f"Pin assignments generated in {GENERATED_FILE_NAME}")

if __name__ == "__main__":
    main()
