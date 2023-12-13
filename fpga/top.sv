module top (
    // inputs
    input           sys_clk,
    input           sync_in,
    input           rst, // synchronous active high reset
    // outputs
    output          sync_out,
    output [1:0]    trans,
    // ft chip
    inout  [7:0]    ft_data,
    input           ft_txen,
    input           ft_rxfn,
    output          ft_rdn,
    output          ft_wrn,
    input           ft_clk,  // sync mode only
    output          ft_oen,  // sync mode only
    output          ft_siwu, // sync mode only
    // for debug only! TODO remove in production
    output [7:0]    phases [0:1],
    output          read_error
);

`define MASTER // comment out if not master

`ifdef MASTER
`endif

// system params
localparam CLK_FREQ = 10_240_000;
localparam OUT_FREQ = 40_000;
localparam NUM_CHANNELS = 2;

// proto245 params
localparam TX_FIFO_SIZE       = 4096;
localparam RX_FIFO_SIZE       = 4096;
localparam SINGLE_CLK_DOMAIN  = 1;
localparam READ_TICKS         = 2;
localparam WRITE_TICKS        = 2;
localparam TX_FIFO_LOAD_W     = $clog2(TX_FIFO_SIZE) + 1;
localparam RX_FIFO_LOAD_W     = $clog2(RX_FIFO_SIZE) + 1;
localparam DATA_W             = 8;

// misc regs
// logic [$clog2(CLK_FREQ/OUT_FREQ)-1:0] phases [0:NUM_CHANNELS-1];
// logic   read_error;
logic   clk;

// proto245 regs
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

assign ft_oen  = 1'b1;
assign ft_data = ft_rdn ? ft_dout : 'z;
assign ft_din  = ft_data;

// TODO replace with pulse generator
assign sync_out = 0;

// TODO replace with pwm module
assign trans[0] = 0;
assign trans[1] = 0;

receiver #(
    .CLK_FREQ       (CLK_FREQ),
    .OUT_FREQ       (OUT_FREQ),
    .NUM_CHANNELS       (NUM_CHANNELS),
    .TX_FIFO_LOAD_W     (TX_FIFO_LOAD_W),
    .RX_FIFO_LOAD_W     (RX_FIFO_LOAD_W)
) receiver (
    // internal inputs
    .clk,
    .rst,
    // internal outputs
    .phases,
    .read_error,
    // proto245 interface
    // RX: Host -> FPGA
    .rxfifo_data,
    .rxfifo_valid,
    .rxfifo_load,
    .rxfifo_empty,
    .rxfifo_rd,
    // TX: FPGA -> Host
    .txfifo_load,
    .txfifo_full,
    .txfifo_wr,
    .txfifo_data
);

proto245a #(
    .DATA_W            (DATA_W),
    .TX_FIFO_SIZE      (TX_FIFO_SIZE),
    .RX_FIFO_SIZE      (RX_FIFO_SIZE),
    .SINGLE_CLK_DOMAIN (SINGLE_CLK_DOMAIN),
    .READ_TICKS        (READ_TICKS),
    .WRITE_TICKS       (WRITE_TICKS)
) proto245 (
    // FT interface (routes to IO)
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
    // inputs
    .rxfifo_clk   (clk),
    .rxfifo_rst   (rst),
    .rxfifo_rd    (rxfifo_rd),      // RX FIFO read enable
    // outputs
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

// TODO generate new ip to match crystal on v1 board
pll50 pll (
    .refclk   (sys_clk),
    .rst      (rst),
    .outclk_0 (clk), // 10.24MHz
    .locked   ()
);

endmodule: top