module de1_soc_top(
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
    output [3:0]        trans,
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

logic sys_clk, rst;
logic [7:0] phases [0:3];
logic read_error;

assign sys_clk = CLOCK_50;
assign rst = ~KEY[3];
assign LEDR[7:0] = phases[0];
assign LEDR[8] = read_error;
assign HEX0 = 7'b1111111;
assign HEX1 = 7'b1111111;
assign HEX2 = 7'b1111111;
assign HEX3 = 7'b1111111;
assign HEX4 = 7'b1111111;
assign HEX5 = 7'b1111111;

// system clock heartbeat
logic [24:0] sys_heartbeat_cnt;
always_ff @(posedge sys_clk) begin
    if (rst)
        sys_heartbeat_cnt <= '0;
    else
        sys_heartbeat_cnt <= sys_heartbeat_cnt + 1'b1;
end
assign LEDR[9] = sys_heartbeat_cnt[24];

top top(.*);

endmodule: de1_soc_top