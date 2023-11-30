module top #(
    CLK_FREQ, OUT_FREQ, NUM_CHANNELS
)(
    // inputs
    input           clk,
    input           sync_in,
    input           rst, // SYNC ACTIVE HIGH
    // outputs
    output          sync_out,
    output [NUM_CHANNELS-1:0] trans,
    // ft chip
    inout  [7:0]    ft_data,
    input           ft_txen,
    input           ft_rxfn,
    output          ft_rdn,
    output          ft_wrn,
    input           ft_clk,  // SYNC ONLY
    output          ft_oen,  // SYNC ONLY
    output          ft_siwu, // SYNC ONLY
);

`define MASTER // Comment out if not master

`ifdef MASTER
`endif

localparam TX_FIFO_SIZE       = 4096;
localparam RX_FIFO_SIZE       = 4096;
localparam SINGLE_CLK_DOMAIN  = 1;
localparam READ_TICKS         = 2;
localparam WRITE_TICKS        = 2;
localparam TX_FIFO_LOAD_W     = $clog2(TX_FIFO_SIZE) + 1;
localparam RX_FIFO_LOAD_W     = $clog2(RX_FIFO_SIZE) + 1;

logic [DATA_W-1:0] ft_din, ft_dout;

logic                      rxfifo_rd;
logic [DATA_W-1:0]         rxfifo_data;
logic                      rxfifo_valid;
logic [RX_FIFO_LOAD_W-1:0] rxfifo_load;
logic                      rxfifo_empty;
logic [DATA_W-1:0]         txfifo_data;
logic                      txfifo_wr;
logic [TX_FIFO_LOAD_W-1:0] txfifo_load;
logic                      txfifo_full;

proto245a #(
    .DATA_W            (DATA_W),
    .TX_FIFO_SIZE      (TX_FIFO_SIZE),
    .RX_FIFO_SIZE      (RX_FIFO_SIZE),
    .SINGLE_CLK_DOMAIN (SINGLE_CLK_DOMAIN),
    .READ_TICKS        (READ_TICKS),
    .WRITE_TICKS       (WRITE_TICKS)
) proto245 (
    // FT interface - should be routed directly to IO
    .ft_rst   (rst),
    .ft_clk   (clk),
    .ft_rxfn  (ft_rxfn),
    .ft_txen  (ft_txen),
    .ft_din   (ft_din),
    .ft_dout  (ft_dout),
    .ft_rdn   (ft_rdn),
    .ft_wrn   (ft_wrn),
    .ft_siwu  (ft_siwu),
    // RX FIFO (Host -> FTDI chip -> FPGA -> FIFO)
    // Inputs
    .rxfifo_clk   (clk),
    .rxfifo_rst   (rst),
    .rxfifo_rd    (rxfifo_rd),      // RX FIFO read enable
    // Outputs
    .rxfifo_data  (rxfifo_data),    // RX FIFO read data
    .rxfifo_valid (rxfifo_valid),   // RX FIFO read data is valid
    .rxfifo_load  (rxfifo_load),    // RX FIFO load counter
    .rxfifo_empty (rxfifo_empty),   // RX FIFO is empty
    // TX FIFO (FIFO -> FPGA -> FTDI chip -> Host)
    // Inputs
    .txfifo_clk   (clk),
    .txfifo_rst   (rst),
    .txfifo_data  (txfifo_data),    // TX FIFO write data
    .txfifo_wr    (txfifo_wr),      // TX FIFO read enable
    // Outputs
    .txfifo_load  (txfifo_load),    // TX FIFO load counter
    .txfifo_full  (txfifo_full)     // TX FIFO is full
);

assign ft_oen  = 1'b1;
assign ft_data = ft_rdn ? ft_dout : 'z;
assign ft_din  = ft_data;

endmodule: top