module pwm #(parameter CLK_FREQ, OUT_FREQ) (
    input clk,
    input rst,
    input en,
    input [$clog2(CLK_FREQ/OUT_FREQ)-1:0] cnt,
    input [$clog2(CLK_FREQ/OUT_FREQ)-1:0] phase,
    output logic out
);

localparam CLK_CNT_MAX = CLK_FREQ / OUT_FREQ;

logic [$clog2(CLK_FREQ/OUT_FREQ)-1:0] end_phase;
assign end_phase = phase + CLK_CNT_MAX/2;

typedef enum {
    pwm_off_e,
    pwm_on_e
} pwm_state;

pwm_state fsm_state = pwm_off_e;

always_ff @(posedge clk) begin
    if (rst) begin
        fsm_state <= pwm_off_e;
    end
    else begin
        case (fsm_state)
            pwm_off_e: begin
                if (cnt == phase) begin
                    fsm_state <= pwm_on_e;
                end
            end
            pwm_on_e: begin
                if (cnt == end_phase) begin
                    fsm_state <= pwm_off_e;
                end
            end
        endcase
    end
end

always_comb begin
    case (fsm_state)
        pwm_off_e: begin
            out = 'b0;
        end
        pwm_on_e: begin
            out = en;
        end
    endcase
end
endmodule: pwm