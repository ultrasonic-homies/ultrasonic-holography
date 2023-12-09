module sync_in(
    input clk,
    input rst,
    input sync_in,
    output logic sync_pulse
);

logic sync_in_pipe_1;
logic sync_in_pipe_2;

typedef enum {
    sync_low_e,
    sync_high_e
} sync_in_state;

sync_in_state fsm_state = sync_low_e;

always_ff @(posedge clk) begin
    if (rst) begin
        sync_in_pipe_1 <= 0;
        sync_in_pipe_2 <= 0;
    end
    else begin
        sync_in_pipe_1 <= sync_in;
        sync_in_pipe_2 <= sync_in_pipe_1;
    end
end

always_ff @(posedge clk) begin
    if (rst) begin
        fsm_state <= sync_low_e;
        sync_pulse <= 0;
    end
    else begin
        case (fsm_state)
            sync_low_e: begin
                sync_pulse <= 0;
                if (sync_in_pipe_2) begin
                    fsm_state <= sync_high_e;
                    sync_pulse <= 1;

                end
            end
            sync_high_e: begin
                sync_pulse <= 0;
                if (~sync_in_pipe_2) begin
                    fsm_state <= sync_low_e;
                end
            end
        endcase
    end
end

endmodule: sync_in