`timescale 1ps/1ps

module tb_receiver();

localparam TX_FIFO_LOAD_W = 13; // log2(4096) + 1;
localparam RX_FIFO_LOAD_W = 13;
localparam MOD_CHANNELS = 4;

// Internal Inputs
logic clk;
logic rst;
// Internal Outputs
wire read_error;
wire phase_parse_en;
wire phase_calib_en;
wire [31:0] latest_data;
logic mod_enable;
logic [MOD_CHANNELS-1:0] mod_set;
logic [15:0] mod_half_period;

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
    TX_FIFO_LOAD_W,
    RX_FIFO_LOAD_W,
    MOD_CHANNELS
) dut (.*);

initial begin
    // Reset
    rst = 1;
    rxfifo_data = 'h00;
    rxfifo_valid = 0;
    rxfifo_load = 0;
    rxfifo_empty = 1;
    #4;
    rst = 0;
    #2;
    rxfifo_empty = 0;
    #2;
    rxfifo_valid = 1;
    // Write the command for phase data write
    rxfifo_data = 'h55; // suffix
    #6;
    rxfifo_data = 'h23; // data
    #6;
    rxfifo_data = 'h01;
    #6;
    rxfifo_data = 'h01;
    #6;
    rxfifo_data = 'h00;
    #6;
    rxfifo_data = 'h01; // code
    #6;
    rxfifo_data = 'h00; // suffix
    #6;
    rxfifo_data = 'hAA; // prefix
    #6;
    // Write the command for phase data calibration
    rxfifo_data = 'h55; // suffix
    #6;
    rxfifo_data = 'h00; // data
    #6;
    rxfifo_data = 'h00;
    #6;
    rxfifo_data = 'h00;
    #6;
    rxfifo_data = 'h00;
    #6;
    rxfifo_data = 'h03; // code
    #6;
    rxfifo_data = 'h00; // suffix
    #6;
    rxfifo_data = 'hAA; // prefix
    #6;
    // Write the command for modulation
    rxfifo_data = 'h55; // suffix
    #6;
    rxfifo_data = 'h01; // half_period | enable
    #6;
    rxfifo_data = 'h05; // half_period
    #6;
    rxfifo_data = 'hf0; // channel | half_period
    #6;
    rxfifo_data = 'h00;
    #6;
    rxfifo_data = 'h04; // code
    #6;
    rxfifo_data = 'h00; // suffix
    #6;
    rxfifo_data = 'hAA; // prefix
    #6;
    // Write the command for burst mode
    rxfifo_data = 'h55; // suffix
    #6;
    rxfifo_data = 'h10; // data
    #6;
    rxfifo_data = 'h00;
    #6;
    rxfifo_data = 'h00;
    #6;
    rxfifo_data = 'h00;
    #6;
    rxfifo_data = 'h02; // code
    #6;
    rxfifo_data = 'h00; // suffix
    #6;
    rxfifo_data = 'hAA; // prefix
    #2;
    rxfifo_valid = 0;
    #4;
    rxfifo_valid = 1;
    // burst
    for (int i = 0; i < 'h10; i++) begin
        rxfifo_data = i;
        #2;
    end

end

initial begin
    clk = 0;
    #1;
    forever begin
        #1 clk = ~clk;
    end
end

endmodule: tb_receiver
