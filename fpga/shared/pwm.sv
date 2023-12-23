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
    PWM_OFF_E,
    PWM_ON_E
} pwm_state;

pwm_state fsm_state = PWM_OFF_E;

always_ff @(posedge clk) begin
    if (rst) begin
        fsm_state <= PWM_OFF_E;
    end
    else begin
        case (fsm_state)
            PWM_OFF_E: begin
                if (cnt == phase) begin
                    fsm_state <= PWM_ON_E;
                end
            end
            PWM_ON_E: begin
                if (cnt == end_phase) begin
                    fsm_state <= PWM_OFF_E;
                end
            end
        endcase
    end
end

always_comb begin
    case (fsm_state)
        PWM_OFF_E: begin
            out = 'b0;
        end
        PWM_ON_E: begin
            out = en;
        end
    endcase
end
endmodule: pwm
