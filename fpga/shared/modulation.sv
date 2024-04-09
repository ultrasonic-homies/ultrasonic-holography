module modulation (
    input clk,
    input rst,
    input mod_enable,
    input [15:0] mod_half_period,
    output mod_out
);

localparam COUNTER_WIDTH = 2;

logic [15:0] sync_counter;
logic [COUNTER_WIDTH-1:0] local_counter; // Set MSB freq to 5.12MHz
logic state;
logic mod;

assign mod_out = !mod_enable | mod;

// Local counter
always_ff @(posedge clk) begin
    if (rst) begin
        local_counter <= 'b0;
    end
    else begin
        local_counter <= local_counter + 'b1;
    end
end

always_ff @(posedge clk) begin
    if (rst) begin
        mod <= 'b1;
        state <= 'b0;
        sync_counter <= 'b0;
    end
    else if (mod_half_period == 'b0) begin // Global Disable
        mod <= 'b0;
    end
    else begin
        case (state)
            'b0: begin
                // Detect Sync Rising Edge
                if (local_counter[COUNTER_WIDTH-1]) begin
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
                if (!local_counter[COUNTER_WIDTH-1]) begin
                    state <= 'b0;
                end
            end
        endcase
    end
end
endmodule: modulation