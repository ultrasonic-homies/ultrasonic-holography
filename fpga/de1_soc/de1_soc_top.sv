module de1_soc_top #(
    parameter NUM_CHANNELS = 4
)(
    // peripherals
    input logic         CLOCK_50,
    input logic [3:0]   KEY,
    input logic [9:0]   SW,
    output logic [9:0]  LEDR,
    output logic [6:0]  HEX0,
    output logic [6:0]  HEX1,
    output logic [6:0]  HEX2,
    output logic [6:0]  HEX3,
    output logic [6:0]  HEX4,
    output logic [6:0]  HEX5,
    // general i/o
    input               sync_in,
    output              sync_out,
    output [NUM_CHANNELS-1:0] trans,
    // ft chip
    inout  [7:0]        ft_data,
    input               ft_txen,
    input               ft_rxfn,
    output              ft_rdn,
    output              ft_wrn,
    input               ft_clk,
    output              ft_oen,
    output              ft_siwu
);

logic sys_clk, ext_rst;
logic [7:0] phases_out [0:NUM_CHANNELS-1];
logic read_error;

assign sys_clk = CLOCK_50;
assign ext_rst = ~KEY[3];
assign LEDR[7:0] = phases_out[0];
assign LEDR[8] = read_error;
hex_to_7seg hex0_7seg(.in_byte(phases_out[1][3:0]), .display(HEX0));
hex_to_7seg hex1_7seg(.in_byte(phases_out[1][7:4]), .display(HEX1));
hex_to_7seg hex2_7seg(.in_byte(phases_out[2][3:0]), .display(HEX2));
hex_to_7seg hex3_7seg(.in_byte(phases_out[2][7:4]), .display(HEX3));
hex_to_7seg hex4_7seg(.in_byte(phases_out[3][3:0]), .display(HEX4));
hex_to_7seg hex5_7seg(.in_byte(phases_out[3][7:4]), .display(HEX5));

// system clock heartbeat
logic [24:0] sys_heartbeat_cnt;
always_ff @(posedge sys_clk) begin
    if (ext_rst)
        sys_heartbeat_cnt <= '0;
    else
        sys_heartbeat_cnt <= sys_heartbeat_cnt + 1'b1;
end
assign LEDR[9] = sys_heartbeat_cnt[24];

top #(.NUM_CHANNELS(NUM_CHANNELS)) top(.*);

endmodule: de1_soc_top