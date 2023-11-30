`timescale 1ps/1ps

module tb_receiver();

localparam CLK_FREQ = 256;
localparam OUT_FREQ = 1;
localparam NUM_CHANNELS = 10;
localparam TX_FIFO_LOAD_W = 13; // log2(4096) + 1;
localparam RX_FIFO_LOAD_W = 13;

// Internal Inputs
logic clk;
logic rst;
// Internal Outputs
wire [$clog2(CLK_FREQ/OUT_FREQ)-1:0] phases [NUM_CHANNELS];
wire read_error;
// proto245 Interface
logic [7:0]     rxfifo_data;
logic           rxfifo_valid;
logic [RX_FIFO_LOAD_W-1:0] rxfifo_load;
logic           rxfifo_empty;
wire          rxfifo_rd;
// TX: FPGA -> Host
logic [TX_FIFO_LOAD_W-1:0] txfifo_load;
logic           txfifo_full;
wire          txfifo_wr;
wire [7:0]    txfifo_data;

assign txfifo_load = 0;
assign txfifo_full = 0;

receiver #(
    CLK_FREQ,
    OUT_FREQ,
    NUM_CHANNELS,
    TX_FIFO_LOAD_W,
    RX_FIFO_LOAD_W
) dut (.*);

initial begin
    rst = 1;
    rxfifo_data = 'h00;
    rxfifo_valid = 0;
    rxfifo_load = 0;
    rxfifo_empty = 1;
    #2;
    rst = 0;
    #2;
    rxfifo_empty = 0;
    rxfifo_data = 'h01;
    #2;
    rxfifo_valid = 1;
    #2;
    rxfifo_valid = 0;
    rxfifo_data = 'h12;
    #2;
    rxfifo_valid = 1;
    #2;
    rxfifo_data = 'h34;
    rxfifo_valid = 0;
    #2;
    rxfifo_valid = 1;
    rxfifo_empty = 0;

end

initial begin
    clk = 0;
    
    forever begin
        #1 clk = ~clk;
    end
end

endmodule: tb_receiver