`timescale 10ns/1ps

module tb_top ();

localparam NUM_CHANNELS = 128;
// inputs
logic               sys_clk;
logic               ext_rst; // synchronous active high reset
logic               sync_in;
// outputs
wire        sync_out;
wire [NUM_CHANNELS - 1:0] trans;
// ft chip
wire  [7:0]        ft_data;
logic               ft_txen;
logic               ft_rxfn;
wire              ft_rdn;
wire              ft_wrn;
logic               ft_clk;
wire              ft_oen;
wire              ft_siwu;

top #(.NUM_CHANNELS(NUM_CHANNELS)) dut (.*);

initial begin
    sync_in = 0;
    ft_txen = 0;
    ft_rxfn = 0;
    ft_clk = 0;
    dut.rxfifo_data = 1;
    ext_rst = 0;
    #10;
    ext_rst = 1;
    #10;
    ext_rst = 0;
    #10;

end

initial begin
    sys_clk = 0;
    forever begin
        #1 sys_clk = ~sys_clk;
    end
end

endmodule: tb_top
