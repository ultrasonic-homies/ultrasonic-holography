`timescale 1ps/1ps
module tb_sync_sender();

localparam PHASE_JITTER = 3;
localparam CLK_CNT_W    = 8;
localparam CLK_CNT_MAX  = 256;

logic                   clk;
logic                   rst;
logic                   sync_pulse;
wire [CLK_CNT_W-1:0]    cnt;
logic [CLK_CNT_W-1:0]    test;
wire                    sync_out;
assign test = ~cnt;

sync_sender #(
    .PHASE_JITTER   (PHASE_JITTER),
    .CLK_CNT_W      (CLK_CNT_W),
    .CLK_CNT_MAX    (CLK_CNT_MAX)
) dut(.*);

initial begin
    sync_pulse = 0;
    rst = 1;
    #4;
    rst = 0;
    assert(cnt == 0) else $error("cnt not reset");
    assert(sync_out == 0) else $error("sync out not 0");
    #4; // cnt should be 2
    assert(cnt == 2) else $error("cnt not incrementing correctly");
    assert(sync_out == 1) else $error("sync out not 1");
    sync_pulse = '1;
    #2;
    assert(cnt == 3) else $error("sync did not obey phase jitter leniency");
    assert(sync_out == 1) else $error("sync out not 1");
    sync_pulse = '0;
    #4;
    assert(cnt == 5) else $error("cnt not incrementing correctly");
    assert(sync_out == 1) else $error("sync out not 1");
    sync_pulse = '1;
    #2;
    assert(cnt == 0) else $error("did not sync correctly");
    assert(sync_out == 1) else $error("sync out not 1");
    dut.cnt = 'hfc;
    sync_pulse = '0;
    #2;
    assert(cnt == 'hfd) else $error("cnt not incrementing correctly");
    assert(sync_out == 0) else $error("sync out did not assert at cnt > max/2");
    sync_pulse = '1;
    #2;
    assert(cnt == 'hfe) else $error("sync did not obey phase jitter leniency");
    assert(sync_out == 0) else $error("sync out did not assert at cnt > max/2");
    sync_pulse = '0;
    #4;
    assert(cnt == 0) else $error("cnt did not rollover");
    assert(sync_out == 1) else $error("sync out did not deassert on rollover");
end

initial begin
    clk = 0;
    #1;
    forever begin
        #1 clk = ~clk; // Period = 2ps
    end
end

endmodule: tb_sync_sender
