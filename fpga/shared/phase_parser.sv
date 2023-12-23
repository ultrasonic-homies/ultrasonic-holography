module phase_parser #(
    parameter CHANNEL
)(
    input clk,
    input rst,
    input en,
    input [15:0] phase_data,
    output logic [7:0] phase
);

logic [7:0] phase_next;

always_comb begin
    if (en && phase_data[15:8] == CHANNEL) begin
        phase_next = phase_data[7:0];
    end
    else begin
        phase_next = phase;
    end
end

always_ff @(posedge clk) begin
    if (rst) begin
        phase <= '0;
    end
    else begin
        phase <= phase_next;
    end
end

endmodule: phase_parser
