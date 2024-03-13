`timescale 10ns/1ps

module tb_de1_soc_top();

logic         CLOCK_50;
logic [3:0]   KEY;
logic [9:0]   SW;
wire [9:0]  LEDR;
wire [6:0]  HEX0;
wire [6:0]  HEX1;
wire [6:0]  HEX2;
wire [6:0]  HEX3;
wire [6:0]  HEX4;
wire [6:0]  HEX5;

// general i/o
logic               sync_in;
wire              sync_out;
wire [3:0]        trans;
// ft chip
wire  [7:0]        ft_data;
logic               ft_txen;
logic               ft_rxfn;
wire              ft_rdn;
wire              ft_wrn;
logic               ft_clk;
wire              ft_oen;
wire              ft_siwu;

assign SW = 10'b0;

de1_soc_top dut(.*);

initial begin
    KEY[2:0] = 'b111;
    sync_in = 0;
    ft_txen = 0;
    ft_rxfn = 0;
    ft_clk = 0;
    dut.top.rxfifo_data = 1;
    KEY[3] = 0;
    #10;
    KEY[3] = 1;
    #10;
    dut.top.pwm_en = '{1, 1, 1, 1};
    dut.top.phase_calibration.phase_calibration = '{0, 0, 0, 0};
    #50000;
end

initial begin
    CLOCK_50 = 0;
    forever begin
        #1 CLOCK_50 = ~CLOCK_50;
    end
end

endmodule: tb_de1_soc_top