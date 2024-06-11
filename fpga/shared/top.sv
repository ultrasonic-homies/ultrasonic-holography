module top #(
    parameter NUM_CHANNELS = 128
)(
    // inputs
    input               sys_clk,
    input               ext_rst, // synchronous active high reset
    input               sync_in,
    // outputs
    output logic        sync_out,
    output [NUM_CHANNELS-1:0] trans,
    // ft chip
    inout  [7:0]        ft_data,
    input               ft_txen,
    input               ft_rxfn,
    output              ft_rdn,
    output              ft_wrn,
    input               ft_clk,  // sync mode only
    output              ft_oen,  // sync mode only
    output              ft_siwu // sync mode only
    // // for debug only! TODO remove in production
    // output logic [7:0]  phases_out [NUM_CHANNELS],
    // output              read_error
);

/* HEADER FILE */

// `include "de0_cv.svh"
// `include "de1_soc.svh"
`include "cyclone10_lp.svh"

/** PROTO245 REGS **/

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

/** SYSTEM REGS **/

logic [CLK_CNT_W-1:0]   phases_in [NUM_CHANNELS];
logic [CLK_CNT_W-1:0]   phase_calibration [NUM_CHANNELS];
logic [CLK_CNT_W-1:0]   phases_intermediate [NUM_CHANNELS];

logic                   read_error;
logic [CLK_CNT_W-1:0]    phases_out [NUM_CHANNELS];
logic                   sys_rst = 'b1; // synchronous active high reset
logic [1:0]             sys_reset_cnt = '0;
logic                   pwm_rst = 'b1; // synchronous active high reset
logic [1:0]             pwm_reset_cnt = '0;
logic [CLK_CNT_W-1:0]   pwm_cnt;
logic                   pwm_en [NUM_CHANNELS] = '{NUM_CHANNELS {0}};
logic                   phase_parse_en;
logic                   phase_calib_en;
logic [15:0]            mod_half_period;
logic                   mod_enable;
logic [MOD_CHANNELS-1:0]mod_set;
logic [31:0]            latest_data;
wire                    sync_pulse;
wire [MOD_CHANNELS-1:0] mod_out;


/** LOGIC **/

// initial system reset
always_ff @(posedge sys_clk) begin
    if (sys_reset_cnt < '1) begin
        sys_rst       <= 1'b1;
        sys_reset_cnt <= sys_reset_cnt + 1'b1;
    end else begin
        sys_rst       <= ext_rst;
    end
end

// initial pwm reset
always_ff @(posedge pwm_clk) begin
    if (pwm_reset_cnt < '1) begin
        pwm_rst       <= 1'b1;
        pwm_reset_cnt <= pwm_reset_cnt + 1'b1;
    end else begin
        pwm_rst       <= ext_rst;
    end
end

// phase calibration
always_ff @(posedge pwm_clk) begin
    for (int i = 0; i < NUM_CHANNELS; i++) begin
        {phases_out[i], phases_intermediate[i]} <= {phases_intermediate[i], phases_in[i] + phase_calibration[i]};
    end
end

/** SUBMODULES **/

genvar i;
generate
    for (i = 0; i < NUM_CHANNELS; i++) begin:channels
        pwm #(CLK_FREQ, OUT_FREQ) pwm (
            .clk(pwm_clk),
            .rst(pwm_rst),
            .en(pwm_en[i] & mod_out[i % MOD_CHANNELS]),
            .cnt(pwm_cnt),
            .phase(phases_out[i]),
            .out(trans[i])
        );
        phase_parser #(.CHANNEL(i)) phase_parser (
            .clk(sys_clk),
            .rst(sys_rst),
            .phase_parse_en(phase_parse_en),
            .phase_calib_en(phase_calib_en),
            .phase_data(latest_data),
            .phase(phases_in[i]),
            .phase_calibration(phase_calibration[i]),
            .pwm_en(pwm_en[i])
        );
    end
endgenerate

receiver #(
    .TX_FIFO_LOAD_W     (TX_FIFO_LOAD_W),
    .RX_FIFO_LOAD_W     (RX_FIFO_LOAD_W),
    .MOD_CHANNELS       (MOD_CHANNELS)
) receiver (
    // internal inputs
    .clk(sys_clk),
    .rst(sys_rst),
    // internal outputs
    .read_error,
    .phase_parse_en,
    .phase_calib_en,
    .mod_enable,
    .mod_set,
    .mod_half_period,
    .latest_data,
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
    .ft_rst   (sys_rst),
    .ft_clk   (sys_clk),
    .ft_rxfn  (ft_rxfn),
    .ft_txen  (ft_txen),
    .ft_din   (ft_din),
    .ft_dout  (ft_dout),
    .ft_rdn   (ft_rdn),
    .ft_wrn   (ft_wrn),
    .ft_siwu  (ft_siwu),
    // RX FIFO (Host -> FTDI chip -> FPGA -> FIFO)
    // inputs
    .rxfifo_clk   (sys_clk),
    .rxfifo_rst   (sys_rst),
    .rxfifo_rd    (rxfifo_rd),      // RX FIFO read enable
    // outputs
    .rxfifo_data  (rxfifo_data),    // RX FIFO read data
    .rxfifo_valid (rxfifo_valid),   // RX FIFO read data is valid
    .rxfifo_load  (rxfifo_load),    // RX FIFO load counter
    .rxfifo_empty (rxfifo_empty),   // RX FIFO is empty
    // TX FIFO (FIFO -> FPGA -> FTDI chip -> Host)
    // Inputs
    .txfifo_clk   (sys_clk),
    .txfifo_rst   (sys_rst),
    .txfifo_data  (txfifo_data),    // TX FIFO write data
    .txfifo_wr    (txfifo_wr),      // TX FIFO read enable
    // Outputs
    .txfifo_load  (txfifo_load),    // TX FIFO load counter
    .txfifo_full  (txfifo_full)     // TX FIFO is full
);

sync_receiver sync_receiver (
    .clk(pwm_clk),
    .rst(pwm_rst),
    .sync_in,
    .sync_pulse
);

sync_sender #(
    .PHASE_JITTER   (PHASE_JITTER),
    .CLK_CNT_W      (CLK_CNT_W),
    .CLK_CNT_MAX    (CLK_CNT_MAX)
) sync_sender (
    .clk(pwm_clk),
    .rst(pwm_rst),
    .sync_pulse,
    .cnt(pwm_cnt),
    .sync_out
);

genvar ch;
generate
    for (ch = 0; ch < MOD_CHANNELS; ch++) begin:mod_channels
        modulation modulation(
            .clk(pwm_clk),
            .rst(pwm_rst),
            .mod_set(mod_set[ch]),
            .mod_enable,
            .mod_half_period,
            .mod_out(mod_out[ch])
        );
    end
endgenerate

endmodule: top
