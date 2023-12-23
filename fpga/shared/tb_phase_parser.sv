module tb_phase_parser();

localparam NUM_CHANNELS = 4;

logic clk;
logic rst;
logic en;
logic [7:0] phases [NUM_CHANNELS];
logic [15:0] phase_data;

genvar i;
generate
    for (i = 0; i < NUM_CHANNELS; i++) begin:channel
        phase_parser #(
            .CHANNEL(i)
            ) phase_parser (
                .clk,
                .rst,
                .en,
                .phase_data,
                .phase(phases[i])
            );
    end
endgenerate

initial begin
    en = 0;
    rst = 1;
    phase_data = 0;
    #4;
    rst = 0;
    phase_data = 'h0101;
    #2;
    en = 1;
    #2;
    phase_data = 'h0202;
    #2;
    assert (phases[1] == 1) else $error("expected phases[1] == 1, actual %h", phases[1]);
    assert (channel[1].phase_parser.phase == 1) else $error("expected phase_parser.phase[1] == 1, actual %h", channel[1].phase_parser.phase);
    en = 0;
    phase_data = 'h0303;
    #2;
    #2;
    assert (phases[2] == 2) else $error("expected phases[2] == 2, actual %h", phases[2]);
    assert (channel[2].phase_parser.phase == 2) else $error("expected phase_parser.phase[2] == 2, actual %h", channel[2].phase_parser.phase);
    #2;
    assert (phases[3] == 0) else $error("expected phases[3] == 0, actual %h", phases[3]);
    assert (channel[3].phase_parser.phase == 0) else $error("expected phase_parser.phase[3] == 0, actual %h", channel[3].phase_parser.phase);
end

initial begin
    clk = 0;
    #1
    forever begin
        #1 clk = ~clk;
    end
end

endmodule: tb_phase_parser