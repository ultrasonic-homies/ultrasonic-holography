module top #(
    parameter DATA_W = 8
)(
    // dev board
    input        CLOCK_50,
    input  [1:0] KEY,
    output [9:0] LEDR,
    input  [9:0] SW,
    output [6:0] HEX0,
    output [6:0] HEX1,
    output [6:0] HEX2,
    output [6:0] HEX3,
    output [6:0] HEX4,
    output [6:0] HEX5,
    // ft board
    output              ft_oen,
    input               ft_clk,
    output              ft_siwu,
    output              ft_wrn,
    output              ft_rdn,
    input               ft_txen,
    input               ft_rxfn,
    inout  [DATA_W-1:0] ft_data,

    output logic sync_out,
    output [3:0] trans
);

//------------------------------------------------------------------------------
// Clocks and resets
//------------------------------------------------------------------------------
logic sys_clk, pwm_clk;
assign sys_clk = CLOCK_50;

// System synchronous active high reset
logic [5:0] sys_reset_cnt = '0;
logic sys_rst = 1'b1;
always_ff @(posedge sys_clk) begin
    if (sys_reset_cnt < '1) begin
        sys_rst       <= 1'b1;
        sys_reset_cnt <= sys_reset_cnt + 1'b1;
    end else begin
        sys_rst       <= 1'b0;
    end
end

// System synchronous active high reset
logic [5:0] pwm_reset_cnt = '0;
logic pwm_rst = 1'b1;
always_ff @(posedge pwm_clk) begin
    if (pwm_reset_cnt < '1) begin
        pwm_rst       <= 1'b1;
        pwm_reset_cnt <= pwm_reset_cnt + 1'b1;
    end else begin
        pwm_rst       <= 1'b0;
    end
end

// FT domain synchronous active high reset
logic [5:0] ft_reset_cnt = '0;
logic ft_rst = 1'b1;
always_ff @(posedge ft_clk) begin
    if (ft_reset_cnt < '1) begin
        ft_rst       <= 1'b1;
        ft_reset_cnt <= ft_reset_cnt + 1'b1;
    end else begin
        ft_rst       <= 1'b0;
    end
end

//------------------------------------------------------------------------------
// FT245 protocol master
//------------------------------------------------------------------------------
// `define FIFO245_SYNC

`ifdef FIFO245_SYNC
    `include "sync245.svh"
`else
    `include "async245.svh"
`endif

//------------------------------------------------------------------------------
// Test logic
//------------------------------------------------------------------------------

localparam NUM_CHANNELS = 4;
// system params
localparam CLK_FREQ = 10_240_000;
localparam OUT_FREQ = 40_000;
localparam CLK_CNT_MAX = CLK_FREQ / OUT_FREQ;

enum logic [3:0] {
    CMD_WAIT_S,
    CMD_READ_S,
    CMD_PARSE_S,
    TX_TEST_S,
    RX_TEST_S
} fsm_state, fsm_next;

logic [63:0] cmd_shifter, cmd_shifter_next;
logic [7:0] cmd_prefix;
logic [7:0] cmd_suffix;
logic [31:0] cmd_data;
logic [15:0] cmd_code;
logic rxfifo_rd_next;
logic [DATA_W-1:0] txfifo_data_next;
logic txfifo_wr_next;
logic led0_drv, led0_drv_next;
logic [31:0] word_cnt, word_cnt_next;
logic [DATA_W-1:0] golden_data, golden_data_next;
logic dbg_led, dbg_led_next;
logic [7:0] phases [NUM_CHANNELS];
logic [7:0] phases_1 [NUM_CHANNELS];
logic [7:0] phases_2 [NUM_CHANNELS];
logic phase_parser_en, phase_parser_en_next;
logic [23:0] data, data_next;
logic [$clog2(CLK_CNT_MAX)-1:0] cnt;
logic en [NUM_CHANNELS-1:0] = '{NUM_CHANNELS {1}};
genvar i;

always @(posedge pwm_clk) begin
    if(pwm_rst) begin
        cnt <= '0;
        sync_out <= '0;
    end
    else begin
        cnt <= cnt == (CLK_CNT_MAX-1) ? 0 : cnt + 1;
        sync_out <= (cnt < CLK_CNT_MAX/2) ? '1 : '0;
    end
end

generate
    for (i = 0; i < NUM_CHANNELS; i++) begin:channels
        pwm #(CLK_FREQ, OUT_FREQ) pwm (
            .clk(pwm_clk),
            .rst(pwm_rst),
            .en(en[i]),
            .cnt,
            .phase(phases_2[i]),
            .out(trans[i])
        );
        phase_parser #(.CHANNEL(i)) phase_parser(
            .clk(sys_clk),
            .rst(sys_rst),
            .en(phase_parser_en),
            .phase_data(data[15:0]),
            .phase(phases[i])
        );
    end
endgenerate

pll50 pll (
    .refclk   (sys_clk),
    .rst      (),
    .outclk_0 (pwm_clk), // 10.24MHz
    .locked   ()
);

seven_seg seven_seg0(.in_byte(data[3:0]),   .display(HEX0));
seven_seg seven_seg1(.in_byte(data[7:4]),   .display(HEX1));
seven_seg seven_seg2(.in_byte(data[11:8]),  .display(HEX2));
seven_seg seven_seg3(.in_byte(data[15:12]), .display(HEX3));
seven_seg seven_seg4(.in_byte(data[19:16]), .display(HEX4));
seven_seg seven_seg5(.in_byte(data[23:20]), .display(HEX5));

assign {cmd_prefix, cmd_code, cmd_data, cmd_suffix} = cmd_shifter;

always_comb begin
    fsm_next         = fsm_state;
    cmd_shifter_next = cmd_shifter;
    rxfifo_rd_next   = rxfifo_rd;
    txfifo_data_next = txfifo_data;
    txfifo_wr_next   = txfifo_wr;
    led0_drv_next    = led0_drv;
    word_cnt_next    = word_cnt;
    golden_data_next = golden_data;
    dbg_led_next     = dbg_led;
    phase_parser_en_next  = 0;
    data_next        = data;

    case (fsm_state)
        CMD_WAIT_S: begin
            txfifo_wr_next = 1'b0;
            rxfifo_rd_next = 1'b0;
            if (!rxfifo_empty) begin
                rxfifo_rd_next = 1'b1;
                fsm_next       = CMD_READ_S;
            end
        end

        CMD_READ_S: begin
            rxfifo_rd_next = 1'b0;
            if (rxfifo_valid) begin
                cmd_shifter_next = {rxfifo_data, cmd_shifter[63:DATA_W]};
                fsm_next         = CMD_PARSE_S;
            end
        end

        CMD_PARSE_S: begin
            if ((cmd_prefix == 8'hAA) && (cmd_suffix == 8'h55)) begin
                data_next = cmd_data[23:0];
                case (cmd_code)
                    16'hbeef: begin
                        cmd_shifter_next = '0;
                        txfifo_wr_next   = 1'b1;
                        txfifo_data_next = '0;
                        word_cnt_next    = cmd_data;
                        fsm_next         = TX_TEST_S;
                    end
                    16'hcafe: begin
                        cmd_shifter_next = '0;
                        word_cnt_next    = cmd_data;
                        golden_data_next = '0;
                        txfifo_data_next = 8'h42;
                        fsm_next         = RX_TEST_S;
                    end
                    16'h1ed0: begin
                        cmd_shifter_next = '0;
                        led0_drv_next    = cmd_data[0];
                        fsm_next         = CMD_WAIT_S;
                    end
                    16'h0001: begin
                        cmd_shifter_next = '0;
                        phase_parser_en_next = 1;
                        led0_drv_next    = cmd_data[8];
                        fsm_next         = CMD_WAIT_S;
                    end
                    default: begin
                        //do nothing
                    end
                endcase
            end else begin
                fsm_next = CMD_WAIT_S;
            end
        end

        TX_TEST_S: begin
            if (word_cnt == 0) begin
                txfifo_wr_next = 1'b0;
                fsm_next       = CMD_WAIT_S;
            end else if (!txfifo_full) begin
                word_cnt_next    = word_cnt - 1'b1;
                txfifo_data_next = txfifo_data + 1'b1;
            end
        end

        RX_TEST_S: begin
            rxfifo_rd_next = !rxfifo_empty;
            if (rxfifo_valid) begin
                if (word_cnt == 0) begin
                    rxfifo_rd_next = 1'b0;
                    txfifo_wr_next = 1'b1;
                    fsm_next       = CMD_WAIT_S;
                end else begin
                    word_cnt_next = word_cnt - 1'b1;
                end
                txfifo_data_next = (rxfifo_data != golden_data) ?  8'hee : txfifo_data;
                golden_data_next = golden_data + 1'b1;
            end
        end

        default: begin
            //do nothing
        end
   endcase
end

always_ff @(posedge sys_clk) begin
    if (sys_rst) begin
        fsm_state   <= CMD_WAIT_S;
        cmd_shifter <= '0;
        rxfifo_rd   <= 1'b0;
        txfifo_data <= '0;
        txfifo_wr   <= 1'b0;
        led0_drv    <= 1'b0;
        word_cnt    <= '0;
        golden_data <= '0;
        dbg_led     <= 1'b0;
        data        <= '0;
        phase_parser_en <= '0;
    end else begin
        fsm_state   <= fsm_next;
        cmd_shifter <= cmd_shifter_next;
        rxfifo_rd   <= rxfifo_rd_next;
        txfifo_data <= txfifo_data_next;
        txfifo_wr   <= txfifo_wr_next;
        led0_drv    <= led0_drv_next;
        word_cnt    <= word_cnt_next;
        golden_data <= golden_data_next;
        dbg_led     <= dbg_led_next;
        data        <= data_next;
        phase_parser_en <= phase_parser_en_next;
    end
end

// PWM CLOCK DOMAIN

always_ff @(posedge pwm_clk) begin
    phases_1 <= phases;
    phases_2 <= phases_1;
end


// `ifdef FIFO245_SYNC
// assign LEDR[7] = '0;
// `else
// assign LEDR[7] = '1;
// `endif
// assign LEDR[6] = ~ft_wrn;
// assign LEDR[5] = ~ft_rdn;
// assign LEDR[4] = ~ft_txen;
// assign LEDR[3] = ~ft_rxfn;
// assign LEDR[2] = rxfifo_rd;
// assign LEDR[1] = txfifo_wr;
assign LEDR[0] = led0_drv;
assign LEDR[8:1] = phases[0][7:0];



//------------------------------------------------------------------------------
// Heartbeats
//------------------------------------------------------------------------------
localparam HEARTBEAT_CNT_W = 25;

// System clock domain
logic [HEARTBEAT_CNT_W-1:0] sys_heartbeat_cnt;
always_ff @(posedge sys_clk) begin
    if (sys_rst)
        sys_heartbeat_cnt <= '0;
    else
        sys_heartbeat_cnt <= sys_heartbeat_cnt + 1'b1;
end
assign LEDR[9] = sys_heartbeat_cnt[HEARTBEAT_CNT_W-1];

// FT clock domain
// logic [HEARTBEAT_CNT_W-1:0] ft_heartbeat_cnt;
// always_ff @(posedge ft_clk) begin
//     if (ft_rst)
//         ft_heartbeat_cnt <= '0;
//     else
//         ft_heartbeat_cnt <= ft_heartbeat_cnt + 1'b1;
// end
// assign LEDR[8] = ft_heartbeat_cnt[HEARTBEAT_CNT_W-1];

endmodule
