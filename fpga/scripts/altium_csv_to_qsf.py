import pandas as pd
from net_name_map import FPGA_PRIMARY_PINOUT, FPGA_SECONDARY_PINOUT

# Parameters
CSV_NAME = "fpga_primary_pinout"
OUTPUT_FILE_NAME = "cyclone10_10cl010_u256_primary"
NET_NAME_MAP = FPGA_PRIMARY_PINOUT

def main():
    df = pd.read_csv(f'{CSV_NAME}.csv', skiprows=1, usecols=[0, 1, 2])

    lines = [
            "#============================================================\n",
            f"# Generated Pin Assignments for {CSV_NAME}\n",
            "#============================================================\n\n\n",
    ]

    for altium_net_name in NET_NAME_MAP.keys():
        u256_pin = df.loc[df["Net Name"] == altium_net_name]["Pin Designator"].iat[0]
        lines.append(f"set_location_assignment PIN_{u256_pin} -to {NET_NAME_MAP[altium_net_name]}\n")

    num_assignments = len(NET_NAME_MAP)
    with open(f"{OUTPUT_FILE_NAME}.qsf", 'w') as output_file:
        output_file.writelines(lines)
    print(f"{num_assignments} pin assignments from {CSV_NAME}.csv generated in {OUTPUT_FILE_NAME}.qsf")

if __name__ == "__main__":
    main()