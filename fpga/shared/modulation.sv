module modulation (
    input clk,
    input rst,
    input sync,
    input mod_enable,
    input [15:0] mod_half_period,
    output mod_out
);

logic [15:0] sync_counter;
logic state;
logic mod;

assign mod_out = !mod_enable | mod;

always_ff @(posedge clk) begin
    if (rst) begin
        mod <= 'b1;
        state <= 'b0;
    end
    else begin
        case (state)
            'b0: begin
                // Detect Sync Rising Edge
                if (sync) begin
                    state <= 'b1;
                    if (sync_counter +'b1 < mod_half_period) begin
                        sync_counter <= sync_counter + 'b1;
                    end
                    else begin
                        sync_counter <= 'b0;
                        mod <= !mod;
                    end
                end
            end
            'b1: begin
                // Detect Sync Falling Edge
                if (!sync) begin
                    state <= 'b0;
                end
            end
        endcase
    end
end
endmodule: modulation