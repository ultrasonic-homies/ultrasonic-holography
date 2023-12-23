module sync_sender #(
    parameter PHASE_JITTER, CLK_CNT_W, CLK_CNT_MAX
)(
    input                           clk,
    input                           rst,
    input                           sync_pulse,
    output logic [CLK_CNT_W-1:0]    cnt,
    output logic                    sync_out
);

logic [CLK_CNT_W-1:0] cnt_n;
assign cnt_n = ~cnt;

always_ff @(posedge clk) begin
    if (rst) begin
        cnt         <= '0;
        sync_out    <= '0;
    end
    else begin
        if (sync_pulse
            && cnt >= PHASE_JITTER
            && cnt_n >= PHASE_JITTER
        ) begin
            cnt     = '0;
        end
        else begin
            cnt     = cnt == (CLK_CNT_MAX-1) ? 0 : cnt + 1;
        end
        sync_out    = (cnt < CLK_CNT_MAX/2) ? '1 : '0;
    end
end

endmodule: sync_sender
